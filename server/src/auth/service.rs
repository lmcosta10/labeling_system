use serde::Serialize;
use crate::user::model::User;
use anyhow::Result;

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
}

pub async fn handle_user(username: String, password: String) -> Result<LoginResponse, anyhow::Error> {
    // TODO: find user by name fn in repository
    let user = User {
        is_admin: false,
        username: username,
        password: "hey".to_string()
    };

    println!("Usuario {}", user.username);

    if user.password == password {
        Ok(LoginResponse {
            success: true,
            message: "Login successful".to_string(),
        })
    } else {
        Err(anyhow::anyhow!("Invalid credentials"))
    }
}