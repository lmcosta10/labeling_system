use serde::Serialize;

#[derive(Serialize)]
pub struct AIResponse {
    pub success: bool,
    pub ai_response: String
}