use crate::image::repository::{
    get_all_images, get_all_images_by_ids, get_all_images_ids_by_group, get_image_tags, set_delete_tag_request, set_edit_tag_request, set_new_tag_request};
use crate::image::model::{Image, ImgResponse, TagResponse};
use anyhow::Result;
use crate::auth;
use axum::{
    response::{IntoResponse, Response},
    http::{StatusCode, header},
    body::Body
};
use std::fs;

pub async fn get_gallery(token: String) -> Vec<Image> {
    let username = auth::repository::get_username_from_session(token);

    let is_admin = auth::repository::get_is_admin_from_username(username.clone());

    if is_admin {
        get_all_images().unwrap() // TODO: replace unwrap
    }
    else {
        let group = auth::repository::get_group_from_username(username.clone());

        let img_ids = get_all_images_ids_by_group(group);

        get_all_images_by_ids(img_ids).unwrap() // TODO: replace unwrap
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

pub async fn get_image(imgpath: String) -> impl IntoResponse {
    let bytes = fs::read(format!("database/images/{}", imgpath)).unwrap(); // TODO: replace unwrap

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/png")
        .body(Body::from(bytes))
        .unwrap() // TODO: replace unwrap
}
