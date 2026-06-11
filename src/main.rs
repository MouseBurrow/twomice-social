mod errors;
mod routes;
pub(crate) mod service;

use axum::routing::{get, post};
use axum::Router;
use config::health::health_response;
use config::server;
use routes::friend_requests::{
    accept_friend_request, list_incoming_requests, list_outgoing_requests, send_friend_request,
};
use routes::friends::list_friends;

async fn health() -> axum::Json<serde_json::Value> {
    health_response("social")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::serve(
        "social",
        Router::new()
            .route("/health", get(health))
            .route("/friend-request", post(send_friend_request))
            .route("/friend-accept", post(accept_friend_request))
            .route("/friend-requests/incoming", get(list_incoming_requests))
            .route("/friend-requests/outgoing", get(list_outgoing_requests))
            .route("/friends", get(list_friends)),
    )
    .await
}
