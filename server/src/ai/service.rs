use crate::ai::model::{AIResponse, AISuggestResponse};
use reqwest::multipart;
use std::fs;

pub async fn get_ai_suggestion() -> Result<AIResponse, anyhow::Error> {
    // TODO: for any image; using this one as placeholder
    let image_bytes = fs::read("database/images/rustcrab.png")?;

    let part = multipart::Part::bytes(image_bytes)
        .file_name("database/images/rustcrab.png")
        .mime_str("image/png")?;

    let form = multipart::Form::new().part("file", part);

    let client = reqwest::Client::new();
    let res = client
        .post("http://ai-model:5000/suggest")
        .multipart(form)
        .send()
        .await?
        .error_for_status()?;
    
    let ai_json: AISuggestResponse = res.json().await?;

    Ok(AIResponse{ success: true, ai_response: ai_json.suggestion })
}
