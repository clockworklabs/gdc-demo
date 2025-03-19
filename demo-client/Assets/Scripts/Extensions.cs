using System.Collections;
using System.Collections.Generic;
using SpacetimeDB.Types;
using UnityEngine;

public static class Extensions
{
    public static Vector2 ToVector2(this StdbVector2 vec2)
    {
        return new Vector2(vec2.X, vec2.Y);
    }
}
