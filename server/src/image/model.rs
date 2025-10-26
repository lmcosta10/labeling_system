#[derive(serde::Serialize)]
pub struct Image {
    pub id: u32,
    pub url: String,
    pub tags: String
}
