use std::fs::File;
use std::env;

use crate::image::model;

pub fn get_image_tags(id: u32) -> Result<model::Image, anyhow::Error> {
    let filename = env::var("IMAGE_DB_FILENAME").unwrap(); // TODO: replace unwrap

    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        if record[0].parse::<u32>().unwrap() == id {
            return Ok(model::Image { // TODO: organize database
                id: record[0].parse::<u32>().unwrap(), // TODO: replace unwrap
                url: record[1].to_string(),
                tags: record[2].to_string()
            })
        }
    }
    Err(anyhow::anyhow!("Did not find image"))
}
