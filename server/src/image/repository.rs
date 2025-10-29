use std::fs::{File, OpenOptions};
use std::env;
use csv::{ReaderBuilder, WriterBuilder};
use crate::image::model::Image;

#[derive(serde::Serialize)]
pub struct TagList {
    pub img_id: u32,
    pub tags_names: Vec<String>
}

pub fn get_all_images() -> Result<Vec<Image>, anyhow::Error> {
    let filename = env::var("IMAGE_DB_FILENAME").unwrap(); // TODO: replace unwrap

    let file = File::open(&filename)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut found_images: Vec<Image> = Vec::new();

    for result in rdr.records() {
        let record = result?;

        let img_id = record[0].parse::<u32>().unwrap();
        
        found_images.push(
            Image{
                id: img_id,
                url: record[1].to_string(),
            }
        );
    }
    Ok(found_images)
}

pub fn get_all_images_by_group(group: i32) -> Result<Vec<Image>, anyhow::Error> {
    let img_ids = get_all_images_ids_by_group(group);

    let filename = env::var("IMAGE_DB_FILENAME").unwrap(); // TODO: replace unwrap

    let file = File::open(&filename).unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    let mut found_images: Vec<Image> = Vec::new();

    for result in rdr.records() {
        let record = result.unwrap();

        let img_id = record[0].parse::<u32>().unwrap();
        
        if img_ids.contains(&img_id) {
            found_images.push(
                Image{
                    id: img_id,
                    url: record[1].to_string(),
                }
            );
        }
    }
    Ok(found_images)
}

pub fn get_all_images_ids_by_group(group: i32) -> Vec<u32> {
    let filename = env::var("IMGGROUP_DB_FILENAME").unwrap(); // TODO: replace unwrap

    let file = File::open(&filename).unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    let mut found_images_ids: Vec<u32> = Vec::new();

    for result in rdr.records() {
        let record = result.unwrap();

        let img_group = record[1].parse::<i32>().unwrap();
        
        if img_group == group {
            found_images_ids.push(record[0].parse::<u32>().unwrap())
        }
    }
    found_images_ids
}

pub fn get_image_tags(id: u32) -> Result<TagList, anyhow::Error> {
    let filename = env::var("TAGS_DB_FILENAME").unwrap(); // TODO: replace unwrap

    let file = File::open(&filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut found_tags_names: Vec<String> = Vec::new();

    for result in rdr.records() {
        let record = result?;

        let record_id = record[0].parse::<u32>().unwrap();

        if record_id == id {
            found_tags_names.push(record[1].to_string());
        }
    }
    // TODO: handle error
    Ok(TagList {
        img_id: id,
        tags_names: found_tags_names
    })
}

// Set requests functions: by ChatGPT ---
pub fn set_new_tag_request(id: u32, tag_name: String) -> Result<u8, anyhow::Error> {
    let filename = env::var("TAGREQUESTS_DB_FILENAME")?; 

    // === 1) Read existing keys ===
    let mut max_req_key: u32 = 0;

    // CSV may not exist yet → OK
    if let Ok(file) = std::fs::File::open(&filename) {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file);

        for result in rdr.records() {
            let record = result?;
            if let Ok(k) = record.get(0).unwrap_or("0").parse::<u32>() {
                if k > max_req_key {
                    max_req_key = k;
                }
            }
        }
    }

    // === 2) New req_key ===
    let new_req_key = max_req_key + 1;

    // === 3) Append new record ===
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filename)?;

    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    wtr.write_record(&[
        new_req_key.to_string(),
        id.to_string(),
        "add".to_string(),
        "".to_string(),
        tag_name,
        "1".to_string(),
    ])?;

    wtr.flush()?;

    Ok(1)
}

pub fn set_edit_tag_request(id: u32, tag_name: String, new_name: String) -> Result<u8, anyhow::Error> {
    let filename = env::var("TAGREQUESTS_DB_FILENAME")?; 

    // === 1) Read existing keys ===
    let mut max_req_key: u32 = 0;

    // CSV may not exist yet → OK
    if let Ok(file) = std::fs::File::open(&filename) {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file);

        for result in rdr.records() {
            let record = result?;
            if let Ok(k) = record.get(0).unwrap_or("0").parse::<u32>() {
                if k > max_req_key {
                    max_req_key = k;
                }
            }
        }
    }

    // === 2) New req_key ===
    let new_req_key = max_req_key + 1;

    // === 3) Append new record ===
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filename)?;

    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    wtr.write_record(&[
        new_req_key.to_string(),
        id.to_string(),
        "edit".to_string(),
        tag_name,
        new_name,
        "1".to_string(),
    ])?;

    wtr.flush()?;

    Ok(1)
}

pub fn set_delete_tag_request(id: u32, tag_name: String) -> Result<u8, anyhow::Error> {
let filename = env::var("TAGREQUESTS_DB_FILENAME")?; 

    // === 1) Read existing keys ===
    let mut max_req_key: u32 = 0;

    // CSV may not exist yet → OK
    if let Ok(file) = std::fs::File::open(&filename) {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file);

        for result in rdr.records() {
            let record = result?;
            if let Ok(k) = record.get(0).unwrap_or("0").parse::<u32>() {
                if k > max_req_key {
                    max_req_key = k;
                }
            }
        }
    }

    // === 2) New req_key ===
    let new_req_key = max_req_key + 1;

    // === 3) Append new record ===
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filename)?;

    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    wtr.write_record(&[
        new_req_key.to_string(),
        id.to_string(),
        "delete".to_string(),
        tag_name,
        "".to_string(),
        "1".to_string(),
    ])?;

    wtr.flush()?;

    Ok(1)
}
// ---

pub fn get_username_from_session(token: String) -> String {
    let filename = env::var("SESSION_DB_FILENAME").unwrap(); // TODO: replace unwrap

    let file = File::open(&filename).unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    let mut username = String::new();

    for result in rdr.records() {
        let record = result.unwrap();

        if record[1].to_string() == token {
            username = record[0].to_string();
        }
    }
    username
}

pub fn get_group_from_username(username: String) -> i32 {
    let filename = env::var("USERGROUP_DB_FILENAME").unwrap(); // TODO: replace unwrap

    let file = File::open(&filename).unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    let mut group = -1; // -1 for when the user is not in any group

    for result in rdr.records() {
        let record = result.unwrap();

        if record[0].to_string() == username {
            group = record[1].parse::<i32>().unwrap();
        }
    }
    group
}

pub fn get_is_admin_from_username(username: String) -> bool {
    let filename = env::var("USER_DB_FILENAME").unwrap(); // TODO: replace unwrap

    let file = File::open(&filename).unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    let mut is_admin = false;

    for result in rdr.records() {
        let record = result.unwrap();

        if record[0].to_string() == username {
            is_admin = record[2].parse::<u32>().unwrap() > 0;
        }
    }
    is_admin
}

pub fn get_image_from_id(id: u32) -> Image {
    let filename = env::var("IMAGE_DB_FILENAME").unwrap(); // TODO: replace unwrap

    let file = File::open(&filename).unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result.unwrap();

        if record[0].parse::<u32>().unwrap() == id {
            return Image { id: id, url: record[1].to_string() };
        }
    }
    Image { id: 0, url: "".to_string() } // TODO: handle error
}
