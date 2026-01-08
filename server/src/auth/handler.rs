use axum::{Json, http::StatusCode};
use crate::auth::service;
use crate::auth::model::{UserLoginInfo, LoginResponse};

pub async fn login_user(
    Json(payload): Json<UserLoginInfo>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    match service::handle_user(payload.username, payload.password).await {
        Ok(resp) => Ok(Json(resp)),
        Err(err) => Err((StatusCode::UNAUTHORIZED, err.to_string())),
    }
}
