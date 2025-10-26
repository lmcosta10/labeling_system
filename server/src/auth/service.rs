use serde::Serialize;
use crate::{auth::repository::get_user_by_username};
use anyhow::Result;

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
}

pub async fn handle_user(username: String, password: String) -> Result<LoginResponse, anyhow::Error> {
    let user = get_user_by_username(username).unwrap(); // TODO: replace unwrap

    println!("Usuario {} é admin?: {}", user.username, user.is_admin);

    if user.password == password {
        Ok(LoginResponse {
            success: true,
            message: "Login successful".to_string(),
        })
    } else {
        Err(anyhow::anyhow!("Invalid credentials"))
    }
}