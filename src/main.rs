use axum::routing::{get, post};
use axum::{Json, Router};
use config::app_data::AppData;
use config::config::Config;
use config::logger;
use serde_json::json;

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok", "service": "social" }))
}

async fn send_friend_request() -> Json<serde_json::Value> {
    Json(json!({ "status": "stub", "message": "friend request not implemented" }))
}

async fn accept_friend_request() -> Json<serde_json::Value> {
    Json(json!({ "status": "stub", "message": "friend accept not implemented" }))
}

async fn list_friends() -> Json<serde_json::Value> {
    Json(json!({ "status": "stub", "friends": [] }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();

    let config = Config::load("social");
    let app_data = AppData::new(config.clone()).await?;
    let addr = format!("0.0.0.0:{}", config.port);

    let app = Router::new()
        .route("/health", get(health))
        .route("/friend-request", post(send_friend_request))
        .route("/friend-accept", post(accept_friend_request))
        .route("/friends", get(list_friends))
        .with_state(app_data);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
