use axum::extract::Path;
use axum::http::HeaderMap;
use axum::{Json, http::StatusCode};
use crate::user_groups::repository::{UserGroupsResponse, SuccessResponse};
use serde::{Deserialize};
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
) -> Result<Json<SuccessResponse>, (StatusCode, String)> {
    let token = server_utils::extract_token(&headers).ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;
    
    let username = auth::repository::get_username_from_session(token);
    let is_admin = auth::repository::get_is_admin_from_username(username);

    if is_admin {
        match service::set_user_addition_to_group(payload.user, payload.group).await {
            Ok(_resp) => Ok(Json(SuccessResponse { success: true })),
            Err(err) => Err((StatusCode::NO_CONTENT, err.to_string())),
        }
    }
    else {
        Err((StatusCode::UNAUTHORIZED, "Not an admin.".to_string()))
    }
}

pub async fn handle_user_groups_deletion(
    headers: HeaderMap, Json(payload): Json<UserGroupPostInfo>
) -> Result<Json<SuccessResponse>, (StatusCode, String)> {
    let token = server_utils::extract_token(&headers).ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;
    
    let username = auth::repository::get_username_from_session(token);
    let is_admin = auth::repository::get_is_admin_from_username(username);

    if is_admin {
        match service::set_user_deletion_from_group(payload.user, payload.group).await {
            Ok(_resp) => Ok(Json(SuccessResponse { success: true })),
            Err(err) => Err((StatusCode::NO_CONTENT, err.to_string())),
        }
    }
    else {
        Err((StatusCode::UNAUTHORIZED, "Not an admin.".to_string()))
    }
}

pub async fn handle_group_creation(
    headers: HeaderMap
) -> Result<Json<SuccessResponse>, (StatusCode, String)> {
    let token = server_utils::extract_token(&headers).ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;
    
    let username = auth::repository::get_username_from_session(token);
    let is_admin = auth::repository::get_is_admin_from_username(username);

    if is_admin {
        match service::set_group_creation().await {
            Ok(_resp) => Ok(Json(SuccessResponse { success: true })),
            Err(err) => Err((StatusCode::NO_CONTENT, err.to_string())),
        }
    }
    else {
        Err((StatusCode::UNAUTHORIZED, "Not an admin.".to_string()))
    }
}

pub async fn handle_group_deletion(
    Path(group): Path<u32>, headers: HeaderMap
) -> Result<Json<SuccessResponse>, (StatusCode, String)> {
    let token = server_utils::extract_token(&headers).ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;
    
    let username = auth::repository::get_username_from_session(token);
    let is_admin = auth::repository::get_is_admin_from_username(username);

    if is_admin {
        match service::set_group_deletion(group).await {
            Ok(_resp) => Ok(Json(SuccessResponse { success: true })),
            Err(err) => Err((StatusCode::NO_CONTENT, err.to_string())),
        }
    }
    else {
        Err((StatusCode::UNAUTHORIZED, "Not an admin.".to_string()))
    }
}
