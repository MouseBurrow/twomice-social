use crate::errors::SocialError;
use crate::service;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use config::app_data::AppData;
use custom_headers::user_id::UserId;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SendFriendRequestBody {
    pub receiver_id: i64,
}

pub async fn send_friend_request(
    State(app): State<AppData>,
    user_id: UserId,
    Json(body): Json<SendFriendRequestBody>,
) -> Result<(StatusCode, Json<service::FriendRequestData>), SocialError> {
    let request = service::send_friend_request(&app.pool, user_id.into(), body.receiver_id).await?;
    Ok((StatusCode::CREATED, Json(request)))
}

#[derive(Deserialize)]
pub struct AcceptFriendRequestBody {
    pub request_id: i64,
}

pub async fn accept_friend_request(
    State(app): State<AppData>,
    user_id: UserId,
    Json(body): Json<AcceptFriendRequestBody>,
) -> Result<(StatusCode, Json<service::FriendshipData>), SocialError> {
    let (_request, friendship) =
        service::accept_friend_request(&app.pool, user_id.into(), body.request_id).await?;
    Ok((StatusCode::CREATED, Json(friendship)))
}

pub async fn list_incoming_requests(
    State(app): State<AppData>,
    user_id: UserId,
) -> Result<Json<Vec<service::FriendRequestData>>, SocialError> {
    let requests = service::list_incoming_requests(&app.pool, user_id.into()).await?;
    Ok(Json(requests))
}

pub async fn list_outgoing_requests(
    State(app): State<AppData>,
    user_id: UserId,
) -> Result<Json<Vec<service::FriendRequestData>>, SocialError> {
    let requests = service::list_outgoing_requests(&app.pool, user_id.into()).await?;
    Ok(Json(requests))
}
