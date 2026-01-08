use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct PendingTagResponse {
    pub req_key: u32,
    pub img_url: String,
    pub operation: String,
    pub old_name: String,
    pub new_name: String,
    pub pending: bool // TODO: remove (frontend)
}