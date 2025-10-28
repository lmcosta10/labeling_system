use std::fs::{File, OpenOptions};
use std::env;
use csv::WriterBuilder;
use crate::image::model::Image;

#[derive(serde::Serialize)]
pub struct TagList {
    pub img_id: u32,
    pub tags_names: Vec<String>,
    pub tags_approved: Vec<u8>
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

pub fn get_image_tags(id: u32) -> Result<TagList, anyhow::Error> {
    let filename = env::var("TAGS_DB_FILENAME").unwrap(); // TODO: replace unwrap

    let file = File::open(&filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut found_tags_names: Vec<String> = Vec::new();
    let mut found_tags_approved: Vec<u8> = Vec::new();

    for result in rdr.records() {
        let record = result?;

        // Parse the ID from column 0
        if let Some(id_str) = record.get(0) {
            if let Ok(record_id) = id_str.parse::<u32>() {
                if record_id == id {
                    if let Some(tag) = record.get(1) {
                        found_tags_names.push(tag.to_string());
                    }
                    if let Some(approved) = record.get(2) {
                        found_tags_approved.push(approved.parse::<u8>().unwrap());
                    }
                }
            }
        }
    }
    // TODO: handle error
    Ok(TagList {
        img_id: id,
        tags_names: found_tags_names,
        tags_approved: found_tags_approved
    })
}

pub fn set_new_tag(id: u32, tag_name: String) -> Result<u8, anyhow::Error> {
    let filename = env::var("TAGS_DB_FILENAME").unwrap(); // TODO: replace unwrap

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filename)?;

    let mut wtr = WriterBuilder::new()
        .has_headers(false) // false for appending
        .from_writer(file);

    wtr.write_record(&[id.to_string(), tag_name, "0".to_string()])?; // TODO: set non pending for admins

    // TODO: handle error
    Ok(1)
}
