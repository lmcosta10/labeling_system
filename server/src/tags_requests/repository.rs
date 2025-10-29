use anyhow;
use std::fs::File;
use std::env;
use serde::Serialize;

use crate::image::repository::get_image_from_id;

#[derive(Serialize)]
pub struct PendingTag {
    img_url: String,
    operation: String,
    old_name: String,
    new_name: String
}

pub fn get_all_pending_tags () -> Result<Vec<PendingTag>, anyhow::Error> {
    let filename = env::var("TAGREQUESTS_DB_FILENAME").unwrap(); // TODO: replace unwrap

    let file = File::open(&filename)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut found_requests: Vec<PendingTag> = Vec::new();

    for result in rdr.records() {
        let record = result?;

        let img_id = record[0].parse::<u32>().unwrap();

        let image_url = get_image_from_id(img_id).url;
        
        found_requests.push(
            PendingTag{
                img_url: image_url,
                operation: record[1].to_string(),
                old_name: record[2].to_string(),
                new_name: record[3].to_string()
            }
        );
    }
    Ok(found_requests)
}
