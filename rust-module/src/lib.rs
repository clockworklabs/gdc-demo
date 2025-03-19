use spacetimedb::{ReducerContext, Table, SpacetimeType};
use rand::prelude::*;
use std::time::Duration;

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
    for _ in 0..count {
        let radius = rng.gen_range(20.0..50.0);
        let x = rng.gen_range(radius..arena_config.width - radius);
        let y = rng.gen_range(radius..arena_config.height - radius);
        let vx = rng.gen_range(-20.0..20.0);
        let vy = rng.gen_range(-20.0..20.0);
        ctx.db.circle().insert(Circle { 
            circle_id: 0,  // Auto-incremented by SpacetimeDB
            pos: StdbVector2 { x, y }, 
            velocity: StdbVector2 { x: vx, y: vy }, 
            radius 
        });
    }
}

#[spacetimedb::reducer]
pub fn simulate_physics(ctx: &ReducerContext, _timer: PhysicsTimer) {
    // Time step for physics simulation
    const DT: f32 = 1.0 / 60.0;
    let arena_config = ctx.db.arena_config().id().find(0).unwrap();
    
    // Update each circle's position and handle bouncing
    for circle in ctx.db.circle().iter() {
        let mut updated_circle = circle.clone();
        
        // Update position based on velocity
        updated_circle.pos.x += circle.velocity.x * DT;
        updated_circle.pos.y += circle.velocity.y * DT;
        
        // Check for collisions with arena boundaries
        // Right wall
        if updated_circle.pos.x + circle.radius > arena_config.width {
            updated_circle.pos.x = arena_config.width - circle.radius;
            updated_circle.velocity.x = -circle.velocity.x;
        }
        // Left wall
        if updated_circle.pos.x - circle.radius < 0.0 {
            updated_circle.pos.x = circle.radius;
            updated_circle.velocity.x = -circle.velocity.x;
        }
        // Bottom wall
        if updated_circle.pos.y + circle.radius > arena_config.height {
            updated_circle.pos.y = arena_config.height - circle.radius;
            updated_circle.velocity.y = -circle.velocity.y;
        }
        // Top wall
        if updated_circle.pos.y - circle.radius < 0.0 {
            updated_circle.pos.y = circle.radius;
            updated_circle.velocity.y = -circle.velocity.y;
        }
        
        // Update the circle in the database
        ctx.db.circle().circle_id().update(updated_circle);
    }
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

