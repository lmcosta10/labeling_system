use axum::http::HeaderMap;
use axum::{Json, http::StatusCode};
use crate::user_groups::repository::UserGroupsResponse;

use crate::user_groups::service;

pub async fn handle_user_groups_page(
    headers: HeaderMap
) -> Result<Json<Vec<UserGroupsResponse>>, (StatusCode, String)> {
    let token = extract_token(&headers).ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

    match service::get_user_groups_page(token).await {
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