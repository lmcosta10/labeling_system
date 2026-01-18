use axum::http::HeaderMap;
use axum::{Json, http::StatusCode, extract::Path,
response::{Response, IntoResponse}};
use crate::image::service;
use crate::image::model::Image;
use crate::image::model::{PostTagInfo, ImgResponse, TagResponse};
use crate::common::server_utils;

pub async fn handle_image_details(
    Path(id): Path<u32>, headers: HeaderMap
) -> Result<Json<ImgResponse>, (StatusCode, String)> {
    let token = server_utils::extract_token(&headers).ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;
    let is_user = server_utils::check_is_user(token);

    if is_user {
        match service::get_image_data(id).await {
            Ok(resp) => Ok(Json(resp)),
            Err(err) => Err((StatusCode::NO_CONTENT, err.to_string())),
        }
    }
    else {
        Err((StatusCode::UNAUTHORIZED, "Not an user.".to_string()))
    }
}

pub async fn handle_tag_post(
    Path(img_id): Path<u32>, headers: HeaderMap, Json(payload): Json<PostTagInfo>
) -> Result<Json<TagResponse>, (StatusCode, String)> {
    let token = server_utils::extract_token(&headers).ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;
    let is_user = server_utils::check_is_user(token);

    if is_user {
        match service::post_tag_user(img_id, payload.action, payload.tag_name, payload.new_name).await {
            Ok(resp) => Ok(Json(resp)),
            Err(err) => Err((StatusCode::NO_CONTENT, err.to_string())),
        }
    }
    else {
        Err((StatusCode::UNAUTHORIZED, "Not an user.".to_string()))
    }
}

pub async fn handle_gallery(
    headers: HeaderMap
) -> Result<Json<Vec<Image>>, (StatusCode, String)> {
    let token = server_utils::extract_token(&headers).ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

    Ok(Json(service::get_gallery(token).await))
}

pub async fn handle_image(
    Path(imgpath): Path<String>, headers: HeaderMap
) -> Response {
    let token = server_utils::extract_token(&headers).ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string())).unwrap_or_default();
    let is_user = server_utils::check_is_user(token.clone());

    if is_user {
        service::get_image(imgpath).await.into_response()
    }
    else {
        (StatusCode::UNAUTHORIZED, "Not an user.").into_response()
    }
}
