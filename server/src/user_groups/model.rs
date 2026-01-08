use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserGroupPostInfo {
    pub group: u32,
    pub user: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserGroupsResponse {
    pub group: u32,
    pub usernames: Vec<String>
}

#[derive(Serialize)]
pub struct SuccessResponse {
    pub success: bool
}
