use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct AIResponse {
    pub success: bool,
    pub ai_response: String
}

#[derive(Deserialize)]
pub struct AISuggestResponse {
    pub suggestion: String,
}