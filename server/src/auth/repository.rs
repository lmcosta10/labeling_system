use std::fs::File;
use std::env;

use crate::user::model;

pub fn get_user_by_username(username: String) -> Result<model::User, anyhow::Error> {
    let filename = env::var("DB_FILENAME").unwrap();

    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        if record[0] == username.to_string() {
            return Ok(model::User { // TODO: organize database
                is_admin: record[2].parse::<i32>().unwrap() == 1,
                username: record[0].to_string(),
                password: record[1].to_string()
            })
        }
    }
    Err(anyhow::anyhow!("Did not find user"))
}
