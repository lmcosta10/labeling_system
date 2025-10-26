use serde::Serialize;
use crate::{image::repository::get_image_tags};
use anyhow::Result;

#[derive(Serialize)]
pub struct ImgResponse {
    pub success: bool,
    pub message: String,
    pub tags: String
}

pub async fn get_image_data(id: u32) -> Result<ImgResponse, anyhow::Error> {
    let image = get_image_tags(id).unwrap(); // TODO: replace unwrap

    println!("Image tags: {}", image.tags);

    if !image.tags.is_empty() {
        Ok(ImgResponse {
            success: true,
            message: "Image data retrieval successful".to_string(),
            tags: image.tags
        })
    } else {
        Err(anyhow::anyhow!("Error retrieving image data"))
    }
}