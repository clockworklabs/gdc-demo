// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

/* eslint-disable */
/* tslint:disable */
// @ts-nocheck
import {
  AlgebraicType,
  AlgebraicValue,
  BinaryReader,
  BinaryWriter,
  CallReducerFlags,
  ConnectionId,
  DbConnectionBuilder,
  DbConnectionImpl,
  DbContext,
  ErrorContextInterface,
  Event,
  EventContextInterface,
  Identity,
  ProductType,
  ProductTypeElement,
  ReducerEventContextInterface,
  SubscriptionBuilderImpl,
  SubscriptionEventContextInterface,
  SumType,
  SumTypeVariant,
  TableCache,
  TimeDuration,
  Timestamp,
  deepEqual,
} from "@clockworklabs/spacetimedb-sdk";

// Import and reexport all reducer arg types
import { AddCircle } from "./add_circle_reducer.ts";
export { AddCircle };
import { AddCircles } from "./add_circles_reducer.ts";
export { AddCircles };
import { IdentityConnected } from "./identity_connected_reducer.ts";
export { IdentityConnected };
import { IdentityDisconnected } from "./identity_disconnected_reducer.ts";
export { IdentityDisconnected };
import { SetArenaSize } from "./set_arena_size_reducer.ts";
export { SetArenaSize };
import { SimulatePhysics } from "./simulate_physics_reducer.ts";
export { SimulatePhysics };

// Import and reexport all table handle types
import { ArenaConfigTableHandle } from "./arena_config_table.ts";
export { ArenaConfigTableHandle };
import { CircleTableHandle } from "./circle_table.ts";
export { CircleTableHandle };
import { PhysicsTimerTableHandle } from "./physics_timer_table.ts";
export { PhysicsTimerTableHandle };

// Import and reexport all types
import { ArenaConfig } from "./arena_config_type.ts";
export { ArenaConfig };
import { Circle } from "./circle_type.ts";
export { Circle };
import { PhysicsTimer } from "./physics_timer_type.ts";
export { PhysicsTimer };
import { StdbVector2 } from "./stdb_vector_2_type.ts";
export { StdbVector2 };

const REMOTE_MODULE = {
  tables: {
    arena_config: {
      tableName: "arena_config",
      rowType: ArenaConfig.getTypeScriptAlgebraicType(),
    },
    circle: {
      tableName: "circle",
      rowType: Circle.getTypeScriptAlgebraicType(),
      primaryKey: "circle_id",
    },
    physics_timer: {
      tableName: "physics_timer",
      rowType: PhysicsTimer.getTypeScriptAlgebraicType(),
      primaryKey: "scheduled_id",
    },
  },
  reducers: {
    add_circle: {
      reducerName: "add_circle",
      argsType: AddCircle.getTypeScriptAlgebraicType(),
    },
    add_circles: {
      reducerName: "add_circles",
      argsType: AddCircles.getTypeScriptAlgebraicType(),
    },
    identity_connected: {
      reducerName: "identity_connected",
      argsType: IdentityConnected.getTypeScriptAlgebraicType(),
    },
    identity_disconnected: {
      reducerName: "identity_disconnected",
      argsType: IdentityDisconnected.getTypeScriptAlgebraicType(),
    },
    set_arena_size: {
      reducerName: "set_arena_size",
      argsType: SetArenaSize.getTypeScriptAlgebraicType(),
    },
    simulate_physics: {
      reducerName: "simulate_physics",
      argsType: SimulatePhysics.getTypeScriptAlgebraicType(),
    },
  },
  // Constructors which are used by the DbConnectionImpl to
  // extract type information from the generated RemoteModule.
  //
  // NOTE: This is not strictly necessary for `eventContextConstructor` because
  // all we do is build a TypeScript object which we could have done inside the
  // SDK, but if in the future we wanted to create a class this would be
  // necessary because classes have methods, so we'll keep it.
  eventContextConstructor: (imp: DbConnectionImpl, event: Event<Reducer>) => {
    return {
      ...(imp as DbConnection),
      event
    }
  },
  dbViewConstructor: (imp: DbConnectionImpl) => {
    return new RemoteTables(imp);
  },
  reducersConstructor: (imp: DbConnectionImpl, setReducerFlags: SetReducerFlags) => {
    return new RemoteReducers(imp, setReducerFlags);
  },
  setReducerFlagsConstructor: () => {
    return new SetReducerFlags();
  }
}

// A type representing all the possible variants of a reducer.
export type Reducer = never
| { name: "AddCircle", args: AddCircle }
| { name: "AddCircles", args: AddCircles }
| { name: "IdentityConnected", args: IdentityConnected }
| { name: "IdentityDisconnected", args: IdentityDisconnected }
| { name: "SetArenaSize", args: SetArenaSize }
| { name: "SimulatePhysics", args: SimulatePhysics }
;

export class RemoteReducers {
  constructor(private connection: DbConnectionImpl, private setCallReducerFlags: SetReducerFlags) {}

  addCircle(pos: StdbVector2, radius: number) {
    const __args = { pos, radius };
    let __writer = new BinaryWriter(1024);
    AddCircle.getTypeScriptAlgebraicType().serialize(__writer, __args);
    let __argsBuffer = __writer.getBuffer();
    this.connection.callReducer("add_circle", __argsBuffer, this.setCallReducerFlags.addCircleFlags);
  }

  onAddCircle(callback: (ctx: ReducerEventContext, pos: StdbVector2, radius: number) => void) {
    this.connection.onReducer("add_circle", callback);
  }

  removeOnAddCircle(callback: (ctx: ReducerEventContext, pos: StdbVector2, radius: number) => void) {
    this.connection.offReducer("add_circle", callback);
  }

  addCircles(count: number) {
    const __args = { count };
    let __writer = new BinaryWriter(1024);
    AddCircles.getTypeScriptAlgebraicType().serialize(__writer, __args);
    let __argsBuffer = __writer.getBuffer();
    this.connection.callReducer("add_circles", __argsBuffer, this.setCallReducerFlags.addCirclesFlags);
  }

  onAddCircles(callback: (ctx: ReducerEventContext, count: number) => void) {
    this.connection.onReducer("add_circles", callback);
  }

  removeOnAddCircles(callback: (ctx: ReducerEventContext, count: number) => void) {
    this.connection.offReducer("add_circles", callback);
  }

  onIdentityConnected(callback: (ctx: ReducerEventContext) => void) {
    this.connection.onReducer("identity_connected", callback);
  }

  removeOnIdentityConnected(callback: (ctx: ReducerEventContext) => void) {
    this.connection.offReducer("identity_connected", callback);
  }

  onIdentityDisconnected(callback: (ctx: ReducerEventContext) => void) {
    this.connection.onReducer("identity_disconnected", callback);
  }

  removeOnIdentityDisconnected(callback: (ctx: ReducerEventContext) => void) {
    this.connection.offReducer("identity_disconnected", callback);
  }

  setArenaSize(width: number, height: number) {
    const __args = { width, height };
    let __writer = new BinaryWriter(1024);
    SetArenaSize.getTypeScriptAlgebraicType().serialize(__writer, __args);
    let __argsBuffer = __writer.getBuffer();
    this.connection.callReducer("set_arena_size", __argsBuffer, this.setCallReducerFlags.setArenaSizeFlags);
  }

  onSetArenaSize(callback: (ctx: ReducerEventContext, width: number, height: number) => void) {
    this.connection.onReducer("set_arena_size", callback);
  }

  removeOnSetArenaSize(callback: (ctx: ReducerEventContext, width: number, height: number) => void) {
    this.connection.offReducer("set_arena_size", callback);
  }

  simulatePhysics(timer: PhysicsTimer) {
    const __args = { timer };
    let __writer = new BinaryWriter(1024);
    SimulatePhysics.getTypeScriptAlgebraicType().serialize(__writer, __args);
    let __argsBuffer = __writer.getBuffer();
    this.connection.callReducer("simulate_physics", __argsBuffer, this.setCallReducerFlags.simulatePhysicsFlags);
  }

  onSimulatePhysics(callback: (ctx: ReducerEventContext, timer: PhysicsTimer) => void) {
    this.connection.onReducer("simulate_physics", callback);
  }

  removeOnSimulatePhysics(callback: (ctx: ReducerEventContext, timer: PhysicsTimer) => void) {
    this.connection.offReducer("simulate_physics", callback);
  }

}

export class SetReducerFlags {
  addCircleFlags: CallReducerFlags = 'FullUpdate';
  addCircle(flags: CallReducerFlags) {
    this.addCircleFlags = flags;
  }

  addCirclesFlags: CallReducerFlags = 'FullUpdate';
  addCircles(flags: CallReducerFlags) {
    this.addCirclesFlags = flags;
  }

  setArenaSizeFlags: CallReducerFlags = 'FullUpdate';
  setArenaSize(flags: CallReducerFlags) {
    this.setArenaSizeFlags = flags;
  }

  simulatePhysicsFlags: CallReducerFlags = 'FullUpdate';
  simulatePhysics(flags: CallReducerFlags) {
    this.simulatePhysicsFlags = flags;
  }

}

export class RemoteTables {
  constructor(private connection: DbConnectionImpl) {}

  get arenaConfig(): ArenaConfigTableHandle {
    return new ArenaConfigTableHandle(this.connection.clientCache.getOrCreateTable<ArenaConfig>(REMOTE_MODULE.tables.arena_config));
  }

  get circle(): CircleTableHandle {
    return new CircleTableHandle(this.connection.clientCache.getOrCreateTable<Circle>(REMOTE_MODULE.tables.circle));
  }

  get physicsTimer(): PhysicsTimerTableHandle {
    return new PhysicsTimerTableHandle(this.connection.clientCache.getOrCreateTable<PhysicsTimer>(REMOTE_MODULE.tables.physics_timer));
  }
}

export class SubscriptionBuilder extends SubscriptionBuilderImpl<RemoteTables, RemoteReducers, SetReducerFlags> { }

export class DbConnection extends DbConnectionImpl<RemoteTables, RemoteReducers, SetReducerFlags> {
  static builder = (): DbConnectionBuilder<DbConnection, ErrorContext, SubscriptionEventContext> => {
    return new DbConnectionBuilder<DbConnection, ErrorContext, SubscriptionEventContext>(REMOTE_MODULE, (imp: DbConnectionImpl) => imp as DbConnection);
  }
  subscriptionBuilder = (): SubscriptionBuilder => {
    return new SubscriptionBuilder(this);
  }
}

export type EventContext = EventContextInterface<RemoteTables, RemoteReducers, SetReducerFlags, Reducer>;
export type ReducerEventContext = ReducerEventContextInterface<RemoteTables, RemoteReducers, SetReducerFlags, Reducer>;
export type SubscriptionEventContext = SubscriptionEventContextInterface<RemoteTables, RemoteReducers, SetReducerFlags>;
export type ErrorContext = ErrorContextInterface<RemoteTables, RemoteReducers, SetReducerFlags>;
