use axum::extract::Path;
use axum::http::HeaderMap;
use axum::{Json, http::StatusCode};
use crate::tags_requests::repository::PendingTagResponse;

use crate::tags_requests::service;

pub async fn handle_tags_requests_page(
    headers: HeaderMap
) -> Result<Json<Vec<PendingTagResponse>>, (StatusCode, String)> {
    let token = extract_token(&headers).ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

    match service::get_pending_approvals(token).await {
        Ok(resp) => Ok(Json(resp)),
        Err(err) => Err((StatusCode::NO_CONTENT, err.to_string())),
    }
}

pub async fn handle_tag_approval(
    Path(req_key): Path<u32>, headers: HeaderMap
) -> Result<Json<bool>, (StatusCode, String)> {
    let token = extract_token(&headers).ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

    match service::set_approved_tag(token, req_key).await {
        Ok(resp) => Ok(Json(resp)),
        Err(err) => Err((StatusCode::NO_CONTENT, err.to_string())),
    }
}

pub async fn handle_tag_rejection(
    Path(req_key): Path<u32>, headers: HeaderMap
) -> Result<Json<bool>, (StatusCode, String)> {
    let token = extract_token(&headers).ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

    match service::set_rejected_tag(token, req_key).await {
        Ok(resp) => Ok(Json(resp)),
        Err(err) => Err((StatusCode::NO_CONTENT, err.to_string())),
    }
}

fn extract_token(headers: &HeaderMap) -> Option<String> {
    let header = headers.get("Authorization")?;
    let header_str = header.to_str().ok()?;

    // Expect: "Bearer <token>"
    header_str.strip_prefix("Bearer ").map(|t| t.to_string())
}
