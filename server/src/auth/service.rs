use crate::auth::repository::{get_user_by_username,add_session};
use crate::auth::model::LoginResponse;
use anyhow::Result;
use uuid::Uuid;

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