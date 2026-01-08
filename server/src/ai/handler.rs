use axum::{Json, http::StatusCode, http::HeaderMap};
use crate::ai::model::AIResponse;
use crate::common::server_utils;

use crate::ai::service;

pub async fn handle_ai_suggestion_request(
    headers: HeaderMap) -> Result<Json<AIResponse>, (StatusCode, String)> {
    let token = server_utils::extract_token(&headers).ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;
    let is_user = server_utils::check_is_user(token);

    if is_user {
        match service::get_ai_suggestion().await {
            Ok(resp) => Ok(Json(resp)),
            Err(err) => Err((StatusCode::NO_CONTENT, err.to_string())),
        }
    }
    else {
        Err((StatusCode::UNAUTHORIZED, "Not an user.".to_string()))
    }
}
