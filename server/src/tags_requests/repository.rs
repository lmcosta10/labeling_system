use anyhow;
use csv::{ReaderBuilder, WriterBuilder};
use std::fs::{self, File, OpenOptions};
use std::env;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::error::Error;
use std::path::Path;
use tempfile::NamedTempFile;

use crate::image::{repository::get_image_from_id};

#[derive(Debug, Deserialize, Serialize)]
pub struct PendingTag {
    req_key: u32,
    img_id: u32,
    operation: String,
    old_tag: String,
    new_tag: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PendingTagResponse {
    req_key: u32,
    img_url: String,
    operation: String,
    old_name: String,
    new_name: String,
    pending: bool
}

pub fn get_all_pending_tags () -> Result<Vec<PendingTagResponse>, anyhow::Error> {
    let filename = env::var("TAGREQUESTS_DB_FILENAME").unwrap(); // TODO: replace unwrap

    let file = File::open(&filename)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut found_requests: Vec<PendingTagResponse> = Vec::new();

    for result in rdr.records() {
        let record = result?;

        let req_key = record[0].parse::<u32>().unwrap();
        let img_id = record[1].parse::<u32>().unwrap();
        let operation = record[2].to_string();
        let old_name = record[3].to_string();
        let new_name = record[4].to_string();
        let pending = record[5].parse::<u32>().unwrap() == 1;

        let image_url = get_image_from_id(img_id).url;
        
        found_requests.push(
            PendingTagResponse{
                req_key: req_key,
                img_url: image_url,
                operation: operation,
                old_name: old_name,
                new_name: new_name,
                pending: pending
            }
        );
    }
    Ok(found_requests)
}

pub fn remove_tag_request(req_key: u32) -> Result<bool> {
    let filename = env::var("TAGREQUESTS_DB_FILENAME").unwrap(); // TODO: replace unwrap

    let file = File::open(&filename).unwrap();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut row_index: usize = 0;
    let mut found_row = false;

    for result in rdr.records() {
        let record = result.unwrap();

        let cur_req_key= record[0].parse::<u32>().unwrap();

        if req_key != cur_req_key && !found_row {
            row_index += 1;
        } else {
            found_row = true;
        }
    }
    let _ = modify_csv_entry(&filename, row_index, 5, "0");
    Ok(true)
}

pub fn approve_tag_request(req_key: u32) -> Result<bool> {
    let filename = env::var("TAGREQUESTS_DB_FILENAME").unwrap(); // TODO: replace unwrap

    let file = File::open(&filename)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    for result in rdr.records() {
        let record = result?;

        let cur_req_key = record[0].parse::<u32>().unwrap();

        if cur_req_key == req_key {
            let cur_req_type = record[2].to_string();
            if cur_req_type == "add" {
                set_new_tag(record[1].parse::<u32>().unwrap(), record[4].to_string());
            }
            if cur_req_type == "edit" {
                set_new_tag(record[1].parse::<u32>().unwrap(), record[4].to_string());
                delete_tag(record[1].parse::<u32>().unwrap(), record[3].to_string());
            }
            if cur_req_type == "delete" {
                delete_tag(record[1].parse::<u32>().unwrap(), record[3].to_string());
            }
        }
    }
    let _ = remove_tag_request(req_key);
    Ok(true)
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