use anyhow;
use csv::{ReaderBuilder, WriterBuilder}; // TODO: delete
use sqlite;
use std::fs::{self, File, OpenOptions};
use std::env;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::error::Error;
use std::path::Path;
use tempfile::NamedTempFile;

use crate::image::{repository::get_image_from_id};

#[derive(Debug, Deserialize, Serialize)]
pub struct PendingTagResponse {
    req_key: u32,
    img_url: String,
    operation: String,
    old_name: String,
    new_name: String,
    pending: bool // TODO: remove (frontend)
}

pub fn get_all_pending_tags () -> Result<Vec<PendingTagResponse>, anyhow::Error> {
    let conn = sqlite::open("./src/database/labelsys.db")?; // drop method is called implicitly

    let all_tags_query = "SELECT * FROM tagrequests LIMIT 5";
    let mut all_tags_statement = conn.prepare(all_tags_query)?;

    let mut found_requests: Vec<PendingTagResponse> = Vec::new();
    
    while let sqlite::State::Row = all_tags_statement.next()? {
        let req_key_i64: i64 = all_tags_statement.read(0)?;
        let req_key = req_key_i64 as u32;
        let img_id_i64: i64 = all_tags_statement.read(1)?;
        let img_id = img_id_i64 as u32;
        let operation: String = all_tags_statement.read(2)?;
        let old_tag_opt: Option<String> = all_tags_statement.read(3)?;
        let new_tag_opt: Option<String> = all_tags_statement.read(4)?;

        let img_url = get_image_from_id(img_id).url;

        found_requests.push(
            PendingTagResponse{
                req_key: req_key,
                img_url: img_url,
                operation: operation,
                old_name: old_tag_opt.unwrap_or_default(), 
                new_name: new_tag_opt.unwrap_or_default(), 
                pending: true
            }
        );
    }

    Ok(found_requests)
}

pub fn remove_tag_request(req_key: u32) -> Result<bool> {
    let conn = sqlite::open("./src/database/labelsys.db")?; // drop method is called implicitly

    let remove_request_query = format!("DELETE FROM tagrequests WHERE req_key={req_key}");
    let mut _remove_request_statement = conn.execute(remove_request_query)?;

    Ok(true) // TODO: handle errors
}

pub fn approve_tag_request(req_key: u32) -> Result<bool> {
    // Scope: in order to close the connection before calling remove_tag_request(),
    // otherwise the connection to the db is locked
    {
        let conn = sqlite::open("./src/database/labelsys.db")?; // drop method is called implicitly

        let tag_query = format!("SELECT * FROM tagrequests WHERE req_key={req_key}");
        let mut tag_statement = conn.prepare(tag_query)?;
        let _ = tag_statement.next()?;
        
        let img_id_i64: i64 = tag_statement.read(1)?;
        let img_id = img_id_i64 as u32;
        let old_tag_opt: Option<String> = tag_statement.read(3)?;
        let old_tag = old_tag_opt.unwrap_or_default();
        let new_tag_opt: Option<String> = tag_statement.read(4)?;
        let new_tag = new_tag_opt.unwrap_or_default();
        
        let cur_req_type: String = tag_statement.read(2).unwrap(); // TODO: replace unwrap
        if cur_req_type == "add" {
            set_new_tag(img_id, new_tag.clone());
        }
        if cur_req_type == "edit" {
            set_new_tag(img_id, new_tag.clone());
            delete_tag(img_id, old_tag.clone());
        }
        if cur_req_type == "delete" {
            delete_tag(img_id, old_tag.clone()); // TODO: remove cloning?
        }
    }

    let _ = remove_tag_request(req_key);

    Ok(true) // TODO: handle errors
}

fn set_new_tag(image_id: u32, tag_name: String) {
    let filename = env::var("TAGS_DB_FILENAME").unwrap(); // TODO: replace unwrap

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filename).unwrap();

    let mut wtr = WriterBuilder::new()
        .has_headers(false) // false for appending
        .from_writer(file);

    let _ = wtr.write_record(&[image_id.to_string(), tag_name, "1".to_string()]);
}

fn delete_tag(image_id: u32, tag_name: String) {
    let filename = env::var("TAGS_DB_FILENAME").unwrap(); // TODO: replace unwrap

    let file = File::open(&filename).unwrap();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut row_index: usize = 0;
    let mut found_row = false;

    for result in rdr.records() {
        let record = result.unwrap();

        let cur_img_id= record[0].parse::<u32>().unwrap();
        let cur_tag_name = record[1].to_string();

        if image_id != cur_img_id && tag_name != cur_tag_name && !found_row {
            row_index += 1;
        } else {
            found_row = true;
        }
    }

    let _ = modify_csv_entry(&filename, row_index, 2, "0");
}

// modify_csv_entry by ChatGPT
// Modifies a specific cell in a CSV file, overwriting the original file.
fn modify_csv_entry(
    file_path: &str,
    row_index: usize,
    column_index: usize,
    new_value: &str,
) -> Result<(), Box<dyn Error>> {

    let path = Path::new(file_path);
    let parent_dir = path.parent().ok_or("no parent dir")?;
    let temp = NamedTempFile::new_in(parent_dir)?;
    let temp_path = temp.path().to_path_buf();

    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let mut wtr = WriterBuilder::new().from_path(&temp_path)?;

    let headers = rdr.headers()?.clone();
    wtr.write_record(&headers)?;

    for (i, result) in rdr.records().enumerate() {
        let record = result?;
        if i == row_index {
            let mut new_vec: Vec<String> =
                record.iter().map(|s| s.to_string()).collect();

            if column_index < new_vec.len() {
                new_vec[column_index] = new_value.to_string();
            }
            wtr.write_record(&new_vec)?;
        } else {
            wtr.write_record(&record)?;
        }
    }

    wtr.flush()?;
    drop(wtr);
    drop(rdr);

    fs::rename(&temp_path, file_path)?;

    Ok(())
}