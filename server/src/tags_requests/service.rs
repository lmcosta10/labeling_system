use crate::image::repository::{get_username_from_session,
get_is_admin_from_username}; // TODO: organize - put these in /user
use crate::tags_requests::repository::{get_all_pending_tags, PendingTag};

pub async fn get_pending_approvals(token: String) -> Result<Vec<PendingTag>, anyhow::Error> {
    let username = get_username_from_session(token);

    let is_admin = get_is_admin_from_username(username.clone());

    if is_admin {
        get_all_pending_tags()
    } else {
        Err(anyhow::anyhow!("Error retrieving pending tags"))
    }
}