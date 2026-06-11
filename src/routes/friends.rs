use crate::errors::SocialError;
use crate::service;
use axum::extract::State;
use axum::Json;
use config::app_data::AppData;
use custom_headers::user_id::UserId;

pub async fn list_friends(
    State(app): State<AppData>,
    user_id: UserId,
) -> Result<Json<Vec<service::FriendData>>, SocialError> {
    let friends = service::list_friends(&app.pool, user_id.into()).await?;
    Ok(Json(friends))
}
