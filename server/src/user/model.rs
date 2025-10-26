#[derive(serde::Serialize)]
pub struct User {
    pub is_admin: bool,
    pub username: String,
    pub password: String
}
