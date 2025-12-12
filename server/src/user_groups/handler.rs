use axum::http::HeaderMap;
use axum::{Json, http::StatusCode};
use crate::user_groups::repository::{UserGroupsResponse};
use serde::Deserialize;
use crate::common::server_utils;
use crate::auth;

use crate::user_groups::service;

#[derive(Deserialize)]
pub struct UserGroupPostInfo {
    group: u32,
    user: String
}

pub async fn handle_user_groups_page(
    headers: HeaderMap
) -> Result<Json<Vec<UserGroupsResponse>>, (StatusCode, String)> {
    let token = server_utils::extract_token(&headers).ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

    match service::get_user_groups_page(token).await {
        Ok(resp) => Ok(Json(resp)),
        Err(err) => Err((StatusCode::NO_CONTENT, err.to_string())),
    }
}

pub async fn handle_user_groups_addition(
    headers: HeaderMap, Json(payload): Json<UserGroupPostInfo>
) -> Result<Json<u32>, (StatusCode, String)> {
    let token = server_utils::extract_token(&headers).ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;
    
    let username = auth::repository::get_username_from_session(token);
    let is_admin = auth::repository::get_is_admin_from_username(username);

    println!("{}", payload.group);
    println!("{}", payload.user);
    Ok(Json(1)) // TODO
}

pub async fn handle_user_groups_deletion(
    headers: HeaderMap, Json(payload): Json<UserGroupPostInfo>
) -> Result<Json<u32>, (StatusCode, String)> {
    let token = server_utils::extract_token(&headers).ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;
    println!("{} for u.g.deletion", token);

    println!("{}", payload.group);
    println!("{}", payload.user);
    Ok(Json(1)) // TODO
}
