use axum::{Json, http::StatusCode};
use serde::Deserialize;
use crate::auth::service;

#[derive(Deserialize)]
pub struct UserLoginInfo {
    username: String,
    password: String,
}

pub async fn login_user(
    Json(payload): Json<UserLoginInfo>,
) -> Result<Json<service::LoginResponse>, (StatusCode, String)> {
    match service::handle_user(payload.username, payload.password).await {
        Ok(resp) => Ok(Json(resp)),
        Err(err) => Err((StatusCode::UNAUTHORIZED, err.to_string())),
    }
}
