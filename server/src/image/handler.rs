use axum::http::HeaderMap;
use axum::{Json, http::StatusCode, extract::Path};
use serde::Deserialize;
use crate::image::service;
use crate::image::model::Image;

#[derive(Deserialize)]
pub struct PostTagInfo {
    action: String,
    #[serde(rename = "tag")]
    pub tag_name: Option<String>,
    #[serde(rename = "newName")]
    pub new_name: Option<String>
}

pub async fn handle_image(
    Path(id): Path<u32>,
) -> Result<Json<service::ImgResponse>, (StatusCode, String)> {
    match service::get_image_data(id).await {
        Ok(resp) => Ok(Json(resp)),
        Err(err) => Err((StatusCode::NO_CONTENT, err.to_string())),
    }
}

pub async fn handle_tag_post(
    Path(img_id): Path<u32>, Json(payload): Json<PostTagInfo>
) -> Result<Json<service::TagResponse>, (StatusCode, String)> {
    match service::post_tag_user(img_id, payload.action, payload.tag_name, payload.new_name).await {
        Ok(resp) => Ok(Json(resp)),
        Err(err) => Err((StatusCode::NO_CONTENT, err.to_string())),
    }
}

pub async fn handle_gallery(
    headers: HeaderMap
) -> Result<Json<Vec<Image>>, (StatusCode, String)> {
    let token = extract_token(&headers).ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

    Ok(Json(service::get_gallery(token).await))
}

fn extract_token(headers: &HeaderMap) -> Option<String> {
    let header = headers.get("Authorization")?;
    let header_str = header.to_str().ok()?;

    // Expect: "Bearer <token>"
    header_str.strip_prefix("Bearer ").map(|t| t.to_string())
}
