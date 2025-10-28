use serde::Serialize;
use crate::image::repository::{
    get_image_tags, set_new_tag, get_all_images};
use crate::image::model::Image;
use anyhow::Result;

#[derive(Serialize)]
pub struct ImgResponse {
    pub success: bool,
    pub message: String,
    pub tags_names: Vec<String>,
    pub tags_approved: Vec<u8>
}

#[derive(Serialize)]
pub struct TagResponse {
    pub success: bool,
    pub message: String
}

pub async fn get_gallery() -> Vec<Image> {
    get_all_images().unwrap()
}

pub async fn get_image_data(id: u32) -> Result<ImgResponse, anyhow::Error> {
    let tag_list = get_image_tags(id).unwrap(); // TODO: replace unwrap

    // for debugging
    // println!("Image tags: {:?}", tag_list.tags_names);

    if !tag_list.tags_names.is_empty() {
        Ok(ImgResponse {
            success: true,
            message: "Image data retrieval successful".to_string(),
            tags_names: tag_list.tags_names,
            tags_approved: tag_list.tags_approved
        })
    } else {
        Err(anyhow::anyhow!("Error retrieving image data"))
    }
}

pub async fn post_tag_user(img_id: u32, action: String, tag_name: Option<String>, new_name: Option<String>) -> Result<TagResponse, anyhow::Error> {
    match action.as_str() {
        "add" => {
            let _ = set_new_tag(img_id, tag_name.unwrap()); // TODO: handle response
        }
        "edit" => {
            println!("Edit");
        }
        "delete" => {
            println!("Delete");
        }
        _ => return Err(anyhow::anyhow!("Invalid action: {}", action)),
    }
    
    // TODO: handle error
    Ok(TagResponse {
            success: true,
            message: "Tag registered for approval".to_string()
    })
}