use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub is_admin: bool,
    pub token: Option<String>
}

#[derive(Deserialize)]
pub struct UserLoginInfo {
    pub username: String,
    pub password: String,
}