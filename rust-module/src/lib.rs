use spacetimedb::{ReducerContext, Table, SpacetimeType};
use rand::prelude::*;
use std::time::Duration;
use spacetimedb::log_stopwatch::LogStopwatch;

#[derive(SpacetimeType, Clone, Debug)]
pub struct StdbVector2 {
    x: f32,
    y: f32,
}

#[derive(Clone)]
#[spacetimedb::table(name = circle, public)]
pub struct Circle {
    #[primary_key]
    #[auto_inc]
    circle_id: u32,
    pos: StdbVector2,
    velocity: StdbVector2,
    radius: f32
}

#[spacetimedb::table(name = physics_timer, scheduled(simulate_physics))]
pub struct PhysicsTimer {
    #[primary_key]
    #[auto_inc]
    scheduled_id: u64,
    scheduled_at: spacetimedb::ScheduleAt,
}

#[spacetimedb::table(name = arena_config)]
pub struct ArenaConfig {
    #[unique]
    id: u32,
    width: f32,
    height: f32,
}

#[spacetimedb::reducer(init)]
pub fn init(ctx: &ReducerContext) -> Result<(), String> {
    ctx.db
    .physics_timer()
    .insert(PhysicsTimer {
        scheduled_id: 0,
        scheduled_at: spacetimedb::ScheduleAt::Interval(Duration::from_millis(16).into()),
    });

    ctx.db.arena_config().insert(ArenaConfig {
        id: 0,
        width: 100.0,
        height: 100.0,
    });
    Ok(())
}

#[spacetimedb::reducer(client_connected)]
pub fn identity_connected(_ctx: &ReducerContext) {
    // Called everytime a new client connects
}

#[spacetimedb::reducer(client_disconnected)]
pub fn identity_disconnected(_ctx: &ReducerContext) {
    // Called everytime a client disconnects
}

#[spacetimedb::reducer]
pub fn add_circle(ctx: &ReducerContext, pos: StdbVector2, radius: f32) {
    log::info!("Adding circle at {:?} with radius {}", pos, radius);
    ctx.db.circle().insert(Circle { 
        circle_id: 0,  // Auto-incremented by SpacetimeDB
        pos, 
        velocity: StdbVector2 { x: 0.0, y: 0.0 }, 
        radius 
    });
}

#[spacetimedb::reducer]
pub fn add_circles(ctx: &ReducerContext, count: u32) {
    log::info!("Adding {} circles", count);
    let arena_config = ctx.db.arena_config().id().find(0).unwrap();
    let mut rng = ctx.rng();
    let mut existing_circles: Vec<Circle> = ctx.db.circle().iter().collect();
    
    'circle_loop: for _ in 0..count {
        let radius = rng.gen_range(5.0..15.0);
        let max_attempts = 50; // Prevent infinite loops
        
        for _ in 0..max_attempts {
            let x = rng.gen_range(radius..arena_config.width - radius);
            let y = rng.gen_range(radius..arena_config.height - radius);
            let new_pos = StdbVector2 { x, y };
            
            // Check for overlap with existing circles
            let mut has_overlap = false;
            for existing in &existing_circles {
                let dx = new_pos.x - existing.pos.x;
                let dy = new_pos.y - existing.pos.y;
                let distance_squared = dx * dx + dy * dy;
                let min_distance = radius + existing.radius;
                
                if distance_squared < min_distance * min_distance {
                    has_overlap = true;
                    break;
                }
            }
            
            // If no overlap found, create the circle
            if !has_overlap {
                let vx = rng.gen_range(-20.0..20.0);
                let vy = rng.gen_range(-20.0..20.0);
                let new_circle = Circle { 
                    circle_id: 0,  // Auto-incremented by SpacetimeDB
                    pos: new_pos,
                    velocity: StdbVector2 { x: vx, y: vy }, 
                    radius 
                };
                ctx.db.circle().insert(new_circle.clone());
                existing_circles.push(new_circle);
                continue 'circle_loop;
            }
        }
        
        // If we couldn't place the circle after max attempts, log a warning
        log::warn!("Could not find non-overlapping position for circle after {} attempts", max_attempts);
    }
}

#[spacetimedb::reducer]
pub fn simulate_physics(ctx: &ReducerContext, _timer: PhysicsTimer) {
    let watch = LogStopwatch::new("physics");

    // Time step for physics simulation
    const DELTA_TIME: f32 = 1.0 / 60.0;
    const SEPARATION_FACTOR: f32 = 0.5; // How much to separate overlapping circles per frame
    let arena_config = ctx.db.arena_config().id().find(0).unwrap();
    
    // First pass: Update positions and handle wall collisions
    let mut updated_circles: Vec<Circle> = Vec::new();
    for circle in ctx.db.circle().iter() {
        let mut updated_circle = circle.clone();
        
        // Update position based on velocity
        updated_circle.pos.x += circle.velocity.x * DELTA_TIME;
        updated_circle.pos.y += circle.velocity.y * DELTA_TIME;
        
        // Check for collisions with arena boundaries and handle resizing
        // Right wall
        if updated_circle.pos.x + circle.radius > arena_config.width {
            updated_circle.pos.x = arena_config.width - circle.radius;
            updated_circle.velocity.x = -circle.velocity.x.abs(); // Ensure velocity points inward
        }
        // Left wall
        if updated_circle.pos.x - circle.radius < 0.0 {
            updated_circle.pos.x = circle.radius;
            updated_circle.velocity.x = circle.velocity.x.abs(); // Ensure velocity points inward
        }
        // Bottom wall
        if updated_circle.pos.y + circle.radius > arena_config.height {
            updated_circle.pos.y = arena_config.height - circle.radius;
            updated_circle.velocity.y = -circle.velocity.y.abs(); // Ensure velocity points inward
        }
        // Top wall
        if updated_circle.pos.y - circle.radius < 0.0 {
            updated_circle.pos.y = circle.radius;
            updated_circle.velocity.y = circle.velocity.y.abs(); // Ensure velocity points inward
        }
        
        updated_circles.push(updated_circle);
    }
    
    // Second pass: Handle circle-to-circle collisions and overlaps
    let circle_count = updated_circles.len();
    for i in 0..circle_count {
        for j in (i + 1)..circle_count {
            let dx = updated_circles[j].pos.x - updated_circles[i].pos.x;
            let dy = updated_circles[j].pos.y - updated_circles[i].pos.y;
            let distance_squared = dx * dx + dy * dy;
            let min_distance = updated_circles[i].radius + updated_circles[j].radius;
            
            // Check if circles are overlapping
            if distance_squared < min_distance * min_distance {
                let distance = distance_squared.sqrt();
                
                // Normalize collision vector
                let nx = dx / distance;
                let ny = dy / distance;
                
                // Calculate relative velocity
                let dvx = updated_circles[j].velocity.x - updated_circles[i].velocity.x;
                let dvy = updated_circles[j].velocity.y - updated_circles[i].velocity.y;
                let velocity_along_normal = dvx * nx + dvy * ny;
                
                // Separate overlapping circles
                let overlap = min_distance - distance;
                let separation_x = (overlap * nx * SEPARATION_FACTOR);
                let separation_y = (overlap * ny * SEPARATION_FACTOR);
                
                updated_circles[i].pos.x -= separation_x;
                updated_circles[i].pos.y -= separation_y;
                updated_circles[j].pos.x += separation_x;
                updated_circles[j].pos.y += separation_y;
                
                // Only apply collision response if circles are moving toward each other
                if velocity_along_normal < 0.0 {
                    // Calculate impulse for elastic collision
                    let impulse = -velocity_along_normal;
                    let impulse_x = impulse * nx;
                    let impulse_y = impulse * ny;
                    
                    // Update velocities
                    updated_circles[i].velocity.x -= impulse_x;
                    updated_circles[i].velocity.y -= impulse_y;
                    updated_circles[j].velocity.x += impulse_x;
                    updated_circles[j].velocity.y += impulse_y;
                }
            }
        }
    }
    
    // Third pass: Final boundary check after separations
    for circle in &mut updated_circles {
        // Ensure circles stay within bounds after all adjustments
        circle.pos.x = circle.pos.x.clamp(circle.radius, arena_config.width - circle.radius);
        circle.pos.y = circle.pos.y.clamp(circle.radius, arena_config.height - circle.radius);
    }
    
    // Update all circles in the database
    for circle in updated_circles {
        ctx.db.circle().circle_id().update(circle);
    }

    watch.end();
}

#[spacetimedb::reducer]
pub fn set_arena_size(ctx: &ReducerContext, width: f32, height: f32) {
    log::info!("Setting arena size to {}x{}", width, height);
    ctx.db.arena_config().id().update(ArenaConfig {
        id: 0,
        width,
        height
    });
}
