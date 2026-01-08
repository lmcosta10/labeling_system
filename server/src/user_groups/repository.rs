use crate::user_groups::model::UserGroupsResponse;

pub fn get_all_user_groups() -> Result<Vec<UserGroupsResponse>, anyhow::Error> {
    let conn = sqlite::open("./src/database/labelsys.db")?; // drop method is called implicitly

    // Get ALL user groups (in "groups" table), even if they don't have any users
    let user_groups_query = "SELECT * FROM \"groups\" LEFT JOIN user_groups
    ON \"groups\".\"group\" = user_groups.\"group\" ORDER BY \"group\""; // "ORDER BY": TODO: optimize code later
    let mut user_groups_statement = conn.prepare(user_groups_query)?;

    let mut all_groups: Vec<u32> = Vec::new();

    let mut found_user_groups: Vec<UserGroupsResponse> = Vec::new();
    
    while let sqlite::State::Row = user_groups_statement.next()? {
        let username: String = user_groups_statement.read(1).unwrap_or_default(); // TODO: replace unwrap_or_default?
        let group_i64: i64 = user_groups_statement.read(0)?;
        let group = group_i64 as u32;

        // If it's the first user found in a group:
        if !all_groups.contains(&group) { 
            let mut usernames: Vec<String> = Vec::new();
            usernames.push(username);

            found_user_groups.push(
                UserGroupsResponse{
                    group: group,
                    usernames: usernames
                }
            );
            all_groups.push(group); // add group to all_groups (currently found groups in query)
        } // If the group had already been added:
        else {
            all_groups.push(group);

            for found_group in found_user_groups.iter_mut() {
                if found_group.group == group {
                    found_group.usernames.push(username.clone());
                }
            }
        };
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

pub fn add_user_to_group(user: String, group: u32) {
    let conn = sqlite::open("./src/database/labelsys.db").unwrap(); // drop method is called implicitly
    // TODO: replace unwrap

    let new_ug_query = format!("INSERT INTO user_groups (\"username\", \"group\")
    VALUES ('{}','{}')", user, group); // TODO: make it safer (from sql injection)
    let _new_ug_statement = conn.execute(new_ug_query).unwrap(); // TODO: replace unwrap
}

pub fn remove_user_from_group(user: String, group: u32) {
    let conn = sqlite::open("./src/database/labelsys.db").unwrap(); // drop method is called implicitly
    // TODO: replace unwrap

    let deletion_query = format!("DELETE FROM user_groups
    WHERE (\"username\" = '{}' AND \"group\" = '{}')", user, group); // TODO: make it safer (from sql injection)
    let _deletion_statement = conn.execute(deletion_query).unwrap(); // TODO: replace unwrap
}

pub fn add_group() {
    let conn = sqlite::open("./src/database/labelsys.db").unwrap(); // drop method is called implicitly
    // TODO: replace unwrap

    let new_group_query = format!("INSERT INTO \"groups\" (\"group\") VALUES (NULL)"); // TODO: make it safer (from sql injection)
    let _new_group_statement = conn.execute(new_group_query).unwrap(); // TODO: replace unwrap
}

pub fn delete_group(group: u32) {
    let conn = sqlite::open("./src/database/labelsys.db").unwrap(); // drop method is called implicitly
    // TODO: replace unwrap

    let delete_group_query = format!("DELETE FROM \"groups\"
    WHERE (\"group\" = {})", group); // TODO: make it safer (from sql injection)
    let _delete_group_statement = conn.execute(delete_group_query).unwrap(); // TODO: replace unwrap

    // FIXME: update database after group deletion
    // TODO?: groups by name, not number
}
