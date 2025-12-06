use serde::Serialize;
use crate::image::repository::{
    get_all_images, get_all_images_by_group, get_group_from_username,
    get_image_tags, get_username_from_session, set_new_tag_request,
    set_edit_tag_request,set_delete_tag_request,
    get_is_admin_from_username};
use crate::image::model::Image;
use anyhow::Result;

#[derive(Serialize)]
pub struct ImgResponse {
    pub success: bool,
    pub message: String,
    pub tags_names: Vec<String>
}

#[derive(Serialize)]
pub struct TagResponse {
    pub success: bool,
    pub message: String
}

pub async fn get_gallery(token: String) -> Vec<Image> {
    let username = get_username_from_session(token);

    let is_admin = get_is_admin_from_username(username.clone());

    if is_admin {
        get_all_images().unwrap() // TODO: replace unwrap
    }
    else {
        let group = get_group_from_username(username.clone());

        get_all_images_by_group(group).unwrap() // TODO: replace unwrap
    }
}

pub async fn get_image_data(id: u32) -> Result<ImgResponse, anyhow::Error> {
    let tag_list = get_image_tags(id).unwrap(); // TODO: replace unwrap

    if !tag_list.tags_names.is_empty() {
        Ok(ImgResponse {
            success: true,
            message: "Image data retrieval successful".to_string(),
            tags_names: tag_list.tags_names
        })
    } else {
        Err(anyhow::anyhow!("Error retrieving image data"))
    }
}

pub async fn post_tag_user(img_id: u32, action: String, tag_name: Option<String>, new_name: Option<String>) -> Result<TagResponse, anyhow::Error> {
    match action.as_str() {
        "add" => {
            let _ = set_new_tag_request(img_id, tag_name.unwrap()); // TODO: handle response
        }
        "edit" => {
            let _ = set_edit_tag_request(img_id, tag_name.unwrap(), new_name.unwrap());
        }
        "delete" => {
            let _ = set_delete_tag_request(img_id, tag_name.unwrap());
        }
        _ => return Err(anyhow::anyhow!("Invalid action: {}", action)),
    }
    
    // TODO: handle error
    Ok(TagResponse {
            success: true,
            message: "Tag registered for approval".to_string()
    })
}