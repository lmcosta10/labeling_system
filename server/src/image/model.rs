#[derive(serde::Serialize)] // convert struct to json string later
pub struct Image {
    pub id: u32,
    pub url: String,
}