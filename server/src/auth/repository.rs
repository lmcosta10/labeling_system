use sqlite;

use crate::user::model;

pub fn get_user_by_username(username: String) -> Result<model::User, anyhow::Error> {
    let conn = sqlite::open("./src/database/labelsys.db").unwrap(); // drop method is called implicitly
    // TODO: replace unwrap

    let user_query = format!("SELECT * FROM users WHERE username = '{}'", username); // FIXME: make it safer (from sql injection)
    let mut user_statement = conn.prepare(user_query).unwrap(); // TODO: replace unwrap
    
    while let sqlite::State::Row = user_statement.next().unwrap() { // TODO: replace unwrap
        let username: String = user_statement.read(0).unwrap(); // TODO: replace unwrap
        let password: String = user_statement.read(1).unwrap(); // TODO: replace unwrap
        let is_admin_u64: i64 = user_statement.read(2).unwrap(); // TODO: replace unwrap

        return Ok(model::User { // TODO: organize database
                is_admin: is_admin_u64 == 1,
                username: username,
                password: password
            })
    }
    Err(anyhow::anyhow!("Did not find user"))
}

pub fn add_session(username: String, token: String) {
    let conn = sqlite::open("./src/database/labelsys.db").unwrap(); // drop method is called implicitly

    let new_session_query = format!("INSERT INTO sessions (username, token)
    VALUES ('{username}','{token}')"); // FIXME: make it safer (from sql injection)
    let _new_session_statement = conn.execute(new_session_query).unwrap(); // TODO: replace unwrap
}
