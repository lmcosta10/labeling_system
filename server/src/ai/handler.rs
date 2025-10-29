use axum::{Json, http::StatusCode};
use crate::ai::repository::AIResponse;

use crate::ai::service;

pub async fn handle_ai_suggestion_request() -> Result<Json<AIResponse>, (StatusCode, String)> {
    match service::get_ai_suggestion().await {
        Ok(resp) => Ok(Json(resp)),
        Err(err) => Err((StatusCode::NO_CONTENT, err.to_string())),
    }
}