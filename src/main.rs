use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    http::{Response, StatusCode},
    routing::{get, post},
    Form, Router,
};
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::sync::broadcast;
use tower_http::trace::TraceLayer;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Default, Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {}

#[derive(Template)]
#[template(path = "privacy.html")]
struct PrivacyTemplate {}

#[derive(Template)]
#[template(path = "chat.html")]
struct ChatTemplate {
    online: usize,
    room: String,
    wsurl: String,
}

#[derive(Template)]
#[template(path = "msg.html")]
struct MessageTemplate {
    leftside: bool,
    username: String,
    time: String,
    message: String,
}

#[derive(Template)]
#[template(path = "notification.html")]
struct NotificationTemplate {
    notification: String,
}

struct OpenChatState {
    usernames: Mutex<HashMap<String, String>>,
    tx: broadcast::Sender<ChatEvents>,
}

#[derive(Debug, Clone)]
struct ChatMessage {
    sender: String,
    message: String,
    time: u128,
}

#[derive(Debug, Clone)]
enum ChatEvents {
    Joined(String),
    Left(String),
    Message(ChatMessage),
}

#[derive(Deserialize)]
struct JoinForm {
    username: String,
    room: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let usernames = Mutex::new(HashMap::new());
    let (tx, _rx) = broadcast::channel(100);

    let state = Arc::new(OpenChatState { usernames, tx });

    let app = Router::new()
        .route("/", get(index))
        .route("/chat", post(join))
        .route("/online", get(online))
        .route("/messages/:uid", get(wshandler))
        .route("/about", get(about))
        .route("/privacy", get(privacy))
        .nest(
            "/public",
            axum_static::static_router("public").with_state(()),
        )
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> impl IntoResponse {
    IndexTemplate::default()
}

async fn privacy() -> impl IntoResponse {
    PrivacyTemplate {}
}
async fn about() -> impl IntoResponse {
    AboutTemplate {}
}

async fn online(State(state): State<Arc<OpenChatState>>) -> String {
    return state.usernames.lock().unwrap().len().to_string();
}

async fn join(
    State(state): State<Arc<OpenChatState>>,
    Form(input): Form<JoinForm>,
) -> Response<Body> {
    let room = input.room.clone();
    let uuid = Uuid::new_v4();
    if input.username.len() < 4 {
        return IndexTemplate {}.into_response();
    }
    if state
        .usernames
        .lock()
        .unwrap()
        .values()
        .any(|x| input.username == *x)
    {
        info!(username = input.username, "duplicate username");
        return IndexTemplate {}.into_response();
    }

    state
        .usernames
        .lock()
        .unwrap()
        .insert(uuid.to_string(), input.username);

    ChatTemplate {
        online: state.usernames.lock().unwrap().len(),
        room,
        wsurl: format!("/messages/{uuid}"),
    }
    .into_response()
}

async fn wshandler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<OpenChatState>>,
    Path(uid): Path<String>,
) -> Response<Body> {
    let username = state.usernames.lock().unwrap().get(&uid).cloned();
    if let Some(username) = username {
        ws.on_upgrade(|socket| handle_msg(socket, state, username, uid))
            .into_response()
    } else {
        (StatusCode::BAD_REQUEST, "Not a valid uid").into_response()
    }
}

async fn handle_msg(stream: WebSocket, state: Arc<OpenChatState>, username: String, uid: String) {
    let (mut sender, mut reciever) = stream.split();
    let mut rx = state.tx.subscribe();

    state.tx.send(ChatEvents::Joined(username.clone())).unwrap();

    let user = username.clone();
    let mut send_task = tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            let body = match event {
                ChatEvents::Joined(u) => NotificationTemplate {
                    notification: format!("{} joined the chat", u),
                }
                .to_string(),
                ChatEvents::Left(u) => NotificationTemplate {
                    notification: format!("{} left the chat", u),
                }
                .to_string(),
                ChatEvents::Message(m) => MessageTemplate {
                    leftside: m.sender != user,
                    username: m.sender,
                    time: m.time.to_string(),
                    message: m.message,
                }
                .to_string(),
            };

            if let Err(e) = sender.send(Message::Text(body)).await {
                error!(error=%e, "failed to send to sender");
                break;
            }
        }
    });

    let user = username.clone();
    let tx = state.tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(msg))) = reciever.next().await {
            let u = user.clone();
            let v: serde_json::Value = serde_json::from_str(&msg).unwrap();
            if let Some(msg) = v["message"].as_str() {
                tx.send(ChatEvents::Message(ChatMessage {
                    sender: u,
                    message: msg.to_string(),
                    time: SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                }))
                .unwrap();
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    state.tx.send(ChatEvents::Left(username.clone())).unwrap();
    state.usernames.lock().unwrap().remove(&uid);
}
