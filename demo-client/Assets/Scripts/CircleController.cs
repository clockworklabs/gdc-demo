using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

public class CircleController : MonoBehaviour
{
    private Image image;

    // pick a few nice colors that go together
    private Color[] colors = new[]
    {
        new Color(0.25f, 0.65f, 0.91f), // Sky Blue (#41a6e8)
        new Color(0.34f, 0.76f, 0.82f), // Turquoise (#57c2d1)
        new Color(0.95f, 0.77f, 0.36f), // Amber (#f2c45b)
        new Color(0.94f, 0.5f, 0.5f), // Coral (#ef8080)
        new Color(0.61f, 0.47f, 0.73f), // Lavender (#9c78ba)
        new Color(0.4f, 0.85f, 0.54f) // Mint Green (#66d98a)
    };
    
    private void Awake()
    {
        image = GetComponent<Image>();
        image.color = colors[UnityEngine.Random.Range(0, colors.Length)];
    }
}
