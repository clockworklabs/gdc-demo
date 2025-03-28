// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#nullable enable

using System;
using SpacetimeDB.ClientApi;
using System.Collections.Generic;
using System.Runtime.Serialization;

namespace SpacetimeDB.Types
{
    public sealed partial class RemoteReducers : RemoteBase
    {
        public delegate void SetArenaSizeHandler(ReducerEventContext ctx, float width, float height);
        public event SetArenaSizeHandler? OnSetArenaSize;

        public void SetArenaSize(float width, float height)
        {
            conn.InternalCallReducer(new Reducer.SetArenaSize(width, height), this.SetCallReducerFlags.SetArenaSizeFlags);
        }

        public bool InvokeSetArenaSize(ReducerEventContext ctx, Reducer.SetArenaSize args)
        {
            if (OnSetArenaSize == null) return false;
            OnSetArenaSize(
                ctx,
                args.Width,
                args.Height
            );
            return true;
        }
    }

    public abstract partial class Reducer
    {
        [SpacetimeDB.Type]
        [DataContract]
        public sealed partial class SetArenaSize : Reducer, IReducerArgs
        {
            [DataMember(Name = "width")]
            public float Width;
            [DataMember(Name = "height")]
            public float Height;

            public SetArenaSize(
                float Width,
                float Height
            )
            {
                this.Width = Width;
                this.Height = Height;
            }

            public SetArenaSize()
            {
            }

            string IReducerArgs.ReducerName => "set_arena_size";
        }
    }

    public sealed partial class SetReducerFlags
    {
        internal CallReducerFlags SetArenaSizeFlags;
        public void SetArenaSize(CallReducerFlags flags) => SetArenaSizeFlags = flags;
    }
}
