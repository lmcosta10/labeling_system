use crate::image::repository::{get_username_from_session,
    get_is_admin_from_username}; // TODO: organize - put these in /user
use crate::tags_requests::repository::{get_all_pending_tags, remove_tag_request, approve_tag_request, PendingTagResponse};

pub async fn get_pending_approvals(token: String) -> Result<Vec<PendingTagResponse>, anyhow::Error> {
    let username = get_username_from_session(token);

    let is_admin = get_is_admin_from_username(username.clone());

    if is_admin {
        get_all_pending_tags()
    } else {
        Err(anyhow::anyhow!("Error: not an admin"))
    }
}

pub async fn set_approved_tag(token: String, req_key: u32) -> Result<bool, anyhow::Error> {
    let username = get_username_from_session(token);

    let is_admin = get_is_admin_from_username(username.clone());

    if is_admin {
        approve_tag_request(req_key)
    } else {
        Err(anyhow::anyhow!("Error: not an admin"))
    }
}

pub async fn set_rejected_tag(token: String, req_key: u32) -> Result<bool, anyhow::Error> {
    let username = get_username_from_session(token);

    let is_admin = get_is_admin_from_username(username.clone());

    if is_admin {
        remove_tag_request(req_key)
    } else {
        Err(anyhow::anyhow!("Error: not an admin"))
    }
}