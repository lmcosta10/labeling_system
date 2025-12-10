use axum::http::HeaderMap;

pub fn extract_token(headers: &HeaderMap) -> Option<String> {
    let header = headers.get("Authorization")?;
    let header_str = header.to_str().ok()?;

    // Expect: "Bearer <token>"
    header_str.strip_prefix("Bearer ").map(|t| t.to_string())
}

pub fn check_is_user(token: String) -> bool {
    let conn = sqlite::open("./src/database/labelsys.db").unwrap(); // drop method is called implicitly
    // TODO: replace unwrap

    let username_query = format!("SELECT * FROM sessions WHERE token = '{}'", token); // TODO: make it safer (from sql injection)
    let mut username_statement = conn.prepare(username_query).unwrap(); // TODO: replace unwrap

    let mut username = String::new();
    
    while let sqlite::State::Row = username_statement.next().unwrap() { // TODO: replace unwrap
        username = username_statement.read(0).unwrap(); // TODO: replace unwrap
    }
    
    !username.is_empty()
}