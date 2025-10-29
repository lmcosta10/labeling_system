use crate::ai::repository::AIResponse;

pub async fn get_ai_suggestion() -> Result<AIResponse, anyhow::Error> {
    Ok(AIResponse{ success: true, ai_response: "Mock AI suggestion".to_string() })
}