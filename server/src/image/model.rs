use serde::{Serialize, Deserialize};

#[derive(Serialize)] // convert struct to json string later
pub struct Image {
    pub id: u32,
    pub url: String,
}

#[derive(Deserialize)]
pub struct PostTagInfo {
    pub action: String,
    #[serde(rename = "tag")]
    pub tag_name: Option<String>,
    #[serde(rename = "newName")]
    pub new_name: Option<String>
}

#[derive(Serialize)]
pub struct TagList {
    pub img_id: u32,
    pub tags_names: Vec<String>
}

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
