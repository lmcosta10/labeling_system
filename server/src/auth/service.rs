use serde::Serialize;
use crate::auth::repository::{get_user_by_username,add_session};
use anyhow::Result;
use uuid::Uuid;

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub is_admin: bool,
    pub token: Option<String>
}

pub async fn handle_user(username: String, password: String) -> Result<LoginResponse, anyhow::Error> {
    let user = get_user_by_username(username).unwrap(); // TODO: replace unwrap

    if user.password == password {
        let token = Uuid::new_v4().to_string();

        add_session(user.username, token.clone());

        Ok(LoginResponse {
            success: true,
            message: "Login successful".to_string(),
            is_admin: user.is_admin,
            token: Some(token),
        })
    } else {
        Err(anyhow::anyhow!("Invalid credentials"))
    }
}