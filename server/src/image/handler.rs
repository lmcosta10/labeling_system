use axum::{Json, http::StatusCode, extract::Path};
use crate::image::service;

pub async fn handle_image(
    Path(id): Path<u32>,
) -> Result<Json<service::ImgResponse>, (StatusCode, String)> {
    match service::get_image_data(id).await {
        Ok(resp) => Ok(Json(resp)),
        Err(err) => Err((StatusCode::NO_CONTENT, err.to_string())),
    }
}
