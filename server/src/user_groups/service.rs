use crate::user_groups::repository::{UserGroupsResponse, get_all_user_groups};
use crate::user_groups::repository::{get_username_from_session, get_is_admin_from_username,
    add_user_to_group, remove_user_from_group, add_group}; // TODO: remove duplicate functions from image crate

pub async fn get_user_groups_page(token: String) -> Result<Vec<UserGroupsResponse>, anyhow::Error> {
    let username = get_username_from_session(token);

    let is_admin = get_is_admin_from_username(username.clone());

    if is_admin {
        get_all_user_groups()
    } else {
        Err(anyhow::anyhow!("Error: not an admin"))
    }
}

pub async fn set_user_addition_to_group(user: String, group: u32) -> Result<u32, anyhow::Error> {
    add_user_to_group(user, group);
    Ok(1) // TODO: handle errors
}

pub async fn set_user_deletion_from_group(user: String, group: u32) -> Result<u32, anyhow::Error> {
    remove_user_from_group(user, group);
    Ok(1) // TODO: handle errors
}

pub async fn set_group_creation() -> Result<u32, anyhow::Error> {
    add_group();
    Ok(1) // TODO: handle errors
}
