// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#nullable enable

using System;
using SpacetimeDB.BSATN;
using SpacetimeDB.ClientApi;
using System.Collections.Generic;
using System.Runtime.Serialization;

namespace SpacetimeDB.Types
{
    public sealed partial class RemoteTables
    {
        public sealed class PhysicsTimerHandle : RemoteTableHandle<EventContext, PhysicsTimer>
        {
            protected override string RemoteTableName => "physics_timer";

            public sealed class ScheduledIdUniqueIndex : UniqueIndexBase<ulong>
            {
                protected override ulong GetKey(PhysicsTimer row) => row.ScheduledId;

                public ScheduledIdUniqueIndex(PhysicsTimerHandle table) : base(table) { }
            }

            public readonly ScheduledIdUniqueIndex ScheduledId;

            internal PhysicsTimerHandle(DbConnection conn) : base(conn)
            {
                ScheduledId = new(this);
            }

            protected override object GetPrimaryKey(PhysicsTimer row) => row.ScheduledId;
        }

        public readonly PhysicsTimerHandle PhysicsTimer;
    }
}
