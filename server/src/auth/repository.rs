use std::fs::{File, OpenOptions};
use std::env;

use csv::WriterBuilder;

use crate::user::model;

pub fn get_user_by_username(username: String) -> Result<model::User, anyhow::Error> {
    let filename = env::var("USER_DB_FILENAME").unwrap(); // TODO: replace unwrap

    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        if record[0].to_string() == username {
            return Ok(model::User { // TODO: organize database
                is_admin: record[2].parse::<i32>().unwrap() == 1,
                username: record[0].to_string(),
                password: record[1].to_string()
            })
        }
    }
    Err(anyhow::anyhow!("Did not find user"))
}

pub fn add_session(username: String, token: String) {
    let filename = env::var("SESSION_DB_FILENAME").unwrap(); // TODO: replace unwrap

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filename).unwrap();

    let mut wtr = WriterBuilder::new()
        .has_headers(false) // false for appending
        .from_writer(file);

    let _ = wtr.write_record(&[username, token]);
}
