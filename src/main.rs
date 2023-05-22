use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use model::{Handle, Joined};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::SystemTime,
};
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::model::{ChatEvent, ChatEventType, ChatMessage};

mod model;
// Our shared state
struct OpenChatStore {
    // We require unique usernames. This tracks which usernames have been taken.
    usernames: Mutex<HashMap<String, String>>,
    // Channel used to send messages to all connected clients.
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "openchat=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Set up application state for use with with_state().
    let usernames = Mutex::new(HashMap::new());
    let (tx, _rx) = broadcast::channel(100);

    let app_state = Arc::new(OpenChatStore { usernames, tx });

    let app = Router::new()
        .route("/join", post(join))
        .route("/websocket/:cid", get(websocket_handler))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<OpenChatStore>>,
    Path(cid): Path<String>,
) -> Response {
    let username = state.usernames.lock().unwrap().get(&cid).cloned();
    if let Some(username) = username {
        ws.on_upgrade(|socket| websocket(socket, state, username, cid))
            .into_response()
    } else {
        (
            StatusCode::BAD_REQUEST,
            "Not valid communicationId, did you joined?",
        )
            .into_response()
    }
}

async fn websocket(stream: WebSocket, state: Arc<OpenChatStore>, username: String, cid: String) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut rx = state.tx.subscribe();

    // Now send the "joined" message to all subscribers.
    let event = ChatEvent {
        name: ChatEventType::Joined,
        identifier: username.clone(),
        timestamp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    };
    let _ = state.tx.send(serde_json::to_string(&event).unwrap());

    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Clone things we want to pass (move) to the receiving task.
    let tx = state.tx.clone();
    let name = username.clone();

    // Spawn a task that takes messages from the websocket, prepends the user
    // name, and sends them to all broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Add username before message.
            let msg = ChatMessage {
                author: name.to_string(),
                timestamp: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
                body: text.to_string(),
            };
            let _ = tx.send(serde_json::to_string(&msg).unwrap());
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // Send "user left" message (similar to "joined" above).
    let event = ChatEvent {
        name: ChatEventType::Left,
        identifier: username.clone(),
        timestamp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    };
    let _ = state.tx.send(serde_json::to_string(&event).unwrap());

    // Remove username from map so new clients can take it again.
    state.usernames.lock().unwrap().remove(&cid);
}

async fn join(State(state): State<Arc<OpenChatStore>>, Json(handle): Json<Handle>) -> Response {
    let uuid = uuid::Uuid::new_v4().to_string();
    let joined = Joined { cid: uuid.clone() };
    if handle.handle == "admin" {
        return (StatusCode::BAD_REQUEST, "Cannot use reserved username").into_response();
    }
    if state
        .usernames
        .lock()
        .unwrap()
        .values()
        .any(|v| handle.handle == *v)
    {
        return (StatusCode::BAD_REQUEST, "Username is taken in this room").into_response();
    }
    state.usernames.lock().unwrap().insert(uuid, handle.handle);
    (StatusCode::OK, Json(joined)).into_response()
}
