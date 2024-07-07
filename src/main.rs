use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

#[derive(Template)]
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
struct ChatTemplate<'a> {
    room: &'a str,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .route("/", get(index))
        .route("/chat", get(chat))
        .route("/about", get(about))
        .route("/privacy", get(privacy))
        .nest("/public", axum_static::static_router("public"))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> impl IntoResponse {
    IndexTemplate {}
}

async fn privacy() -> impl IntoResponse {
    PrivacyTemplate {}
}
async fn about() -> impl IntoResponse {
    AboutTemplate {}
}

async fn chat() -> impl IntoResponse {
    ChatTemplate {
        room: "General"
    }
}
