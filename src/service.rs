use crate::errors::SocialError;
use chrono::{DateTime, Utc};
use easy_errors::map_sqlx_error;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::{Pool, Postgres};

#[derive(FromRow, Serialize)]
pub struct FriendRequestData {
    pub id: i64,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Serialize)]
pub struct FriendshipData {
    pub id: i64,
    pub user_id: i64,
    pub friend_id: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct FriendData {
    pub user_id: i64,
}

pub async fn send_friend_request(
    pool: &Pool<Postgres>,
    requester_id: i64,
    receiver_id: i64,
) -> Result<FriendRequestData, SocialError> {
    if requester_id == receiver_id {
        return Err(SocialError::CannotFriendSelf);
    }

    let (smaller, larger) = if requester_id < receiver_id {
        (requester_id, receiver_id)
    } else {
        (receiver_id, requester_id)
    };

    let already_friends: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM friendships WHERE user_id = $1 AND friend_id = $2)",
    )
    .bind(smaller)
    .bind(larger)
    .fetch_one(pool)
    .await
    .map_err(map_sqlx_error::<SocialError>)?;

    if already_friends {
        return Err(SocialError::AlreadyFriends);
    }

    let request: FriendRequestData = sqlx::query_as(
        "INSERT INTO friend_requests (sender_id, receiver_id) VALUES ($1, $2) RETURNING id, sender_id, receiver_id, status, created_at, updated_at",
    )
    .bind(requester_id)
    .bind(receiver_id)
    .fetch_one(pool)
    .await
    .map_err(map_sqlx_error::<SocialError>)?;

    Ok(request)
}

pub async fn accept_friend_request(
    pool: &Pool<Postgres>,
    user_id: i64,
    request_id: i64,
) -> Result<(FriendRequestData, FriendshipData), SocialError> {
    let request: Option<FriendRequestData> = sqlx::query_as(
        "SELECT id, sender_id, receiver_id, status, created_at, updated_at FROM friend_requests WHERE id = $1 AND status = 'pending'",
    )
    .bind(request_id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx_error::<SocialError>)?;

    let request = request.ok_or(SocialError::FriendRequestNotFound)?;

    if request.receiver_id != user_id {
        return Err(SocialError::Unauthorized);
    }

    let (smaller, larger) = if request.sender_id < request.receiver_id {
        (request.sender_id, request.receiver_id)
    } else {
        (request.receiver_id, request.sender_id)
    };

    let friendship: FriendshipData = sqlx::query_as(
        "INSERT INTO friendships (user_id, friend_id) VALUES ($1, $2) RETURNING id, user_id, friend_id, created_at",
    )
    .bind(smaller)
    .bind(larger)
    .fetch_one(pool)
    .await
    .map_err(map_sqlx_error::<SocialError>)?;

    let mut updated_request = request;
    sqlx::query("UPDATE friend_requests SET status = 'accepted', updated_at = NOW() WHERE id = $1")
        .bind(request_id)
        .execute(pool)
        .await
        .map_err(map_sqlx_error::<SocialError>)?;

    updated_request.status = "accepted".to_string();

    Ok((updated_request, friendship))
}

pub async fn list_friends(
    pool: &Pool<Postgres>,
    user_id: i64,
) -> Result<Vec<FriendData>, SocialError> {
    let friendships: Vec<FriendshipData> = sqlx::query_as(
        "SELECT id, user_id, friend_id, created_at FROM friendships WHERE user_id = $1 OR friend_id = $1 ORDER BY created_at",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error::<SocialError>)?;

    let friends: Vec<FriendData> = friendships
        .into_iter()
        .map(|f| {
            let friend_user_id = if f.user_id == user_id {
                f.friend_id
            } else {
                f.user_id
            };
            FriendData {
                user_id: friend_user_id,
            }
        })
        .collect();

    Ok(friends)
}

pub async fn list_incoming_requests(
    pool: &Pool<Postgres>,
    user_id: i64,
) -> Result<Vec<FriendRequestData>, SocialError> {
    let requests: Vec<FriendRequestData> = sqlx::query_as(
        "SELECT id, sender_id, receiver_id, status, created_at, updated_at FROM friend_requests WHERE receiver_id = $1 AND status = 'pending' ORDER BY created_at",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error::<SocialError>)?;

    Ok(requests)
}

pub async fn list_outgoing_requests(
    pool: &Pool<Postgres>,
    user_id: i64,
) -> Result<Vec<FriendRequestData>, SocialError> {
    let requests: Vec<FriendRequestData> = sqlx::query_as(
        "SELECT id, sender_id, receiver_id, status, created_at, updated_at FROM friend_requests WHERE sender_id = $1 AND status = 'pending' ORDER BY created_at",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(map_sqlx_error::<SocialError>)?;

    Ok(requests)
}
