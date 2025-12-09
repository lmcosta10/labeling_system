use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserGroupsResponse {
    username: String,
    group: u32
}

pub fn get_all_user_groups() -> Result<Vec<UserGroupsResponse>, anyhow::Error> {
    let conn = sqlite::open("./src/database/labelsys.db")?; // drop method is called implicitly

    let user_groups_query = "SELECT * FROM user_groups";
    let mut user_groups_statement = conn.prepare(user_groups_query)?;

    let mut found_user_groups: Vec<UserGroupsResponse> = Vec::new();
    
    while let sqlite::State::Row = user_groups_statement.next()? {
        let username: String = user_groups_statement.read(0)?;
        let group_i64: i64 = user_groups_statement.read(1)?;
        let group = group_i64 as u32;

        found_user_groups.push(
            UserGroupsResponse{
                username: username,
                group: group
            }
        );
    }

    Ok(found_user_groups)
}

pub fn get_username_from_session(token: String) -> String {
    let conn = sqlite::open("./src/database/labelsys.db").unwrap(); // drop method is called implicitly
    // TODO: replace unwrap

    let username_query = format!("SELECT * FROM sessions WHERE token = '{}'", token); // TODO: make it safer (from sql injection)
    let mut username_statement = conn.prepare(username_query).unwrap(); // TODO: replace unwrap

    let mut username = String::new();
    
    while let sqlite::State::Row = username_statement.next().unwrap() { // TODO: replace unwrap
        username = username_statement.read(0).unwrap(); // TODO: replace unwrap
    }
    username
}

pub fn get_is_admin_from_username(username: String) -> bool {
    let conn = sqlite::open("./src/database/labelsys.db").unwrap(); // drop method is called implicitly
    // TODO: replace unwrap

    let user_query = format!("SELECT * FROM users WHERE username = '{}'", username); // TODO: make it safer (from sql injection)
    let mut user_statement = conn.prepare(user_query).unwrap(); // TODO: replace unwrap

    let mut is_admin = false;
    
    while let sqlite::State::Row = user_statement.next().unwrap() { // TODO: replace unwrap
        let is_admin_u64: i64 = user_statement.read(2).unwrap(); // TODO: replace unwrap
        is_admin = is_admin_u64 == 1
    }

    is_admin
}