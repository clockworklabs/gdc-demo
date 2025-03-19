using System;
using System.Collections.Generic;
using System.Linq;
using System.Linq.Expressions;
using SpacetimeDB;
using SpacetimeDB.Types;
using TMPro;
using UnityEngine;
using UnityEngine.UI;

public class GameManager : MonoBehaviour
{
    const string DEFAULT_SERVER_URL = "http://127.0.0.1:3000";
    const string DEFAULT_MODULE_NAME = "gdc-demo";
    
    public TMP_InputField hostAddressInput;
    public TMP_InputField moduleNameInput;
    public CircleController circleControllerPrefab;
    public Transform newCircleHier;

    public GameObject welcomePanel;
    public GameObject connectPanel;
    public GameObject connectingPanel;

    public static GameManager Instance { get; private set; }
    public static Identity LocalIdentity { get; private set; }
    public static DbConnection Conn { get; private set; }

    private void Start()
    {
        Instance = this;
        Application.targetFrameRate = 120;
        Application.runInBackground = true;
        welcomePanel.SetActive(true);
        connectPanel.SetActive(true);
        connectingPanel.SetActive(false);

        hostAddressInput.text = DEFAULT_SERVER_URL;
        moduleNameInput.text = DEFAULT_MODULE_NAME;
    }

    public void Connect()
    {
        connectPanel.SetActive(false);
        connectingPanel.SetActive(true);
        
        // In order to build a connection to SpacetimeDB we need to register
        // our callbacks and specify a SpacetimeDB server URI and module name.
        var builder = DbConnection.Builder()
            .OnConnect(HandleConnect)
            .OnConnectError(HandleConnectError)
            .OnDisconnect(HandleDisconnect)
            .WithUri(hostAddressInput.text)
            .WithModuleName(moduleNameInput.text);

        // If the user has a SpacetimeDB auth token stored in the Unity PlayerPrefs,
        // we can use it to authenticate the connection.
        // For testing purposes, it is often convenient to comment the following lines out and
        // export an executable for the project using File -> Build Settings.
        // Then, you can run the executable multiple times. Since the executable will not check for
        // a saved auth token, each run of will receive a different Identifier,
        // and their circles will be able to eat each other.
        if (AuthToken.Token != "")
        {
            builder = builder.WithToken(AuthToken.Token);
        }

        // Building the connection will establish a connection to the SpacetimeDB
        // server.
        Conn = builder.Build();
    }

    // Called when we connect to SpacetimeDB and receive our client identity
    void HandleConnect(DbConnection conn, Identity identity, string token)
    {
        Debug.Log("Connected.");
        AuthToken.SaveToken(token);
        LocalIdentity = identity;
        welcomePanel.SetActive(false);

        conn.Db.Circle.OnInsert += OnCircleInsert;
        conn.Db.Circle.OnUpdate += OnCircleUpdate;
        
        // Request all tables
        Conn.SubscriptionBuilder()
            .OnApplied(HandleSubscriptionApplied)
            .SubscribeToAllTables();
    }

    void HandleConnectError(Exception ex)
    {
        Debug.LogError($"Connection error: {ex}");
    }

    void HandleDisconnect(DbConnection _conn, Exception ex)
    {
        Debug.Log("Disconnected.");
        if (ex != null)
        {
            Debug.LogException(ex);
        }
    }

    private void HandleSubscriptionApplied(SubscriptionEventContext ctx)
    {
        Debug.Log("Subscription applied! circles=" + Conn.Db.Circle.Count);
        // OnSubscriptionApplied?.Invoke();
    }

    private Dictionary<uint, CircleController> circles = new();
    private static void OnCircleInsert(EventContext context, Circle insertedValue)
    {
        var newCircle = Instantiate(Instance.circleControllerPrefab, Instance.newCircleHier);
        newCircle.transform.position = insertedValue.Pos.ToVector2();
        var rectTransform = newCircle.GetComponent<RectTransform>();
        rectTransform.sizeDelta = new Vector2(insertedValue.Radius * 2, insertedValue.Radius * 2);
        Instance.circles.Add(insertedValue.CircleId, newCircle);
    }

    private static void OnCircleUpdate(EventContext context, Circle oldCircle, Circle newCircle)
    {
        if (Instance.circles.TryGetValue(newCircle.CircleId, out var controller))
        {
            // set the position of this circle
            controller.transform.position = newCircle.Pos.ToVector2();
        }
    }

    public static bool IsConnected()
    {
        return Conn != null && Conn.IsActive;
    }

    public void Disconnect()
    {
        Conn.Disconnect();
        Conn = null;
    }
    
    private int lastWidth;
    private int lastHeight;

    private void Update()
    {
        if (!IsConnected()) return;
        
        // Tell the server to update the simulation size as the screen changes dimensions
        if (Screen.width != lastWidth || Screen.height != lastHeight)
        {
            Conn.Reducers.SetArenaSize(Screen.width, Screen.height);
            lastWidth = Screen.width;
            lastHeight = Screen.height;
        }
    }
}
