use axum::routing::{get, post};
use axum::Router;
use config::health::health_response;
use config::server;
use serde_json::json;

async fn health() -> axum::Json<serde_json::Value> {
    health_response("social")
}

async fn send_friend_request() -> axum::Json<serde_json::Value> {
    axum::Json(json!({ "status": "stub", "message": "friend request not implemented" }))
}

async fn accept_friend_request() -> axum::Json<serde_json::Value> {
    axum::Json(json!({ "status": "stub", "message": "friend accept not implemented" }))
}

async fn list_friends() -> axum::Json<serde_json::Value> {
    axum::Json(json!({ "status": "stub", "friends": [] }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::serve(
        "social",
        Router::new()
            .route("/health", get(health))
            .route("/friend-request", post(send_friend_request))
            .route("/friend-accept", post(accept_friend_request))
            .route("/friends", get(list_friends)),
    )
    .await
}
