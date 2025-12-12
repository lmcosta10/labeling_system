use sqlite;
use crate::image::model::Image;

#[derive(serde::Serialize)]
pub struct TagList {
    pub img_id: u32,
    pub tags_names: Vec<String>
}

pub fn get_all_images() -> Result<Vec<Image>, anyhow::Error> {
    let conn = sqlite::open("./src/database/labelsys.db")?; // drop method is called implicitly

    let all_images_query = "SELECT * FROM images LIMIT 5";
    let mut all_images_statement = conn.prepare(all_images_query)?;

    let mut found_images: Vec<Image> = Vec::new();
    
    while let sqlite::State::Row = all_images_statement.next()? {
        let img_id_i64: i64 = all_images_statement.read(0)?;
        let img_id = img_id_i64 as u32;
        let img_url: String = all_images_statement.read(1)?;

        found_images.push(
            Image{
                id: img_id,
                url: img_url,
            }
        );
    }

    Ok(found_images)
}

pub fn get_all_images_by_ids(img_ids: Vec<u32>) -> Result<Vec<Image>, anyhow::Error> {
    let conn = sqlite::open("./src/database/labelsys.db")?; // drop method is called implicitly

    let all_images_query = "SELECT * FROM images";
    let mut all_images_statement = conn.prepare(all_images_query)?;

    let mut found_images: Vec<Image> = Vec::new();
    
    while let sqlite::State::Row = all_images_statement.next()? {
        let img_id_i64: i64 = all_images_statement.read(0)?;
        let img_id = img_id_i64 as u32;
        let img_url: String = all_images_statement.read(1)?;

        if img_ids.contains(&img_id) {
            found_images.push(
                Image{
                    id: img_id,
                    url: img_url,
                }
            );
        }
    }

    Ok(found_images)
}

pub fn get_all_images_ids_by_group(group: u32) -> Vec<u32> {
    let conn = sqlite::open("./src/database/labelsys.db").unwrap(); // drop method is called implicitly
    // TODO: replace unwrap

    let images_query = format!("SELECT * FROM image_groups WHERE \"group\" = {}", group); // group is a reserved word
    let mut images_statement = conn.prepare(images_query).unwrap(); // TODO: replace unwrap

    let mut found_images_ids: Vec<u32> = Vec::new();
    
    while let sqlite::State::Row = images_statement.next().unwrap() { // TODO: replace unwrap
        let id_i64: i64 = images_statement.read(0).unwrap(); // TODO: replace unwrap
        let id = id_i64 as u32;
        found_images_ids.push(id);
    }

    found_images_ids
}

pub fn get_image_tags(id: u32) -> Result<TagList, anyhow::Error> {
    let conn = sqlite::open("./src/database/labelsys.db")?; // drop method is called implicitly

    let all_tags_query = format!("SELECT * FROM tags WHERE img_id = {}", id);
    let mut all_tags_statement = conn.prepare(all_tags_query)?;

    let mut found_tags: Vec<String> = Vec::new();
    
    while let sqlite::State::Row = all_tags_statement.next()? {
        let tag: String = all_tags_statement.read(1)?;

        found_tags.push(tag);
    }

    Ok(TagList {
        img_id: id,
        tags_names: found_tags
    })
}

pub fn set_new_tag_request(img_id: u32, tag_name: String) -> Result<u8, anyhow::Error> {
    let conn = sqlite::open("./src/database/labelsys.db")?; // drop method is called implicitly

    // First, get neccessary info:
    // - highest req_key (currently, new key is equal to the highest existing key + 1)
    let highest_key_query = format!("SELECT MAX(req_key) FROM tag_requests");
    let mut highest_key_statement = conn.prepare(highest_key_query)?;
    let _ = highest_key_statement.next()?;
    let mut req_key: i64 = highest_key_statement.read(0)?;
    req_key = req_key + 1;

    // Insert entry
    let new_tag_query = format!("INSERT INTO tag_requests (req_key, img_id, operation, new_tag)
    VALUES ({req_key},{img_id},'add','{tag_name}')"); // TODO: make it safer (from sql injection)
    let _new_tag_statement = conn.execute(new_tag_query)?;
    
    Ok(1) // TODO: handle errors
}

pub fn set_edit_tag_request(img_id: u32, tag_name: String, new_name: String) -> Result<u8, anyhow::Error> {
    let conn = sqlite::open("./src/database/labelsys.db")?; // drop method is called implicitly

    // First, get neccessary info:
    // - highest req_key (currently, new key is equal to the highest existing key + 1)
    let highest_key_query = format!("SELECT MAX(req_key) FROM tag_requests");
    let mut highest_key_statement = conn.prepare(highest_key_query)?;
    let _ = highest_key_statement.next()?;
    let mut req_key: i64 = highest_key_statement.read(0)?;
    req_key = req_key + 1;

    // Insert entry
    let new_edit_tag_query = format!("INSERT INTO tag_requests (req_key, img_id, operation, old_tag, new_tag)
    VALUES ({req_key},{img_id},'edit','{tag_name}', '{new_name}')"); // TODO: make it safer (from sql injection)
    let _new_edit_tag_statement = conn.execute(new_edit_tag_query)?;
    
    Ok(1) // TODO: handle errors
}

pub fn set_delete_tag_request(img_id: u32, tag_name: String) -> Result<u8, anyhow::Error> {
    let conn = sqlite::open("./src/database/labelsys.db")?; // drop method is called implicitly

    // First, get neccessary info:
    // - highest req_key (currently, new key is equal to the highest existing key + 1)
    let highest_key_query = format!("SELECT MAX(req_key) FROM tag_requests");
    let mut highest_key_statement = conn.prepare(highest_key_query)?;
    let _ = highest_key_statement.next()?;
    let mut req_key: i64 = highest_key_statement.read(0)?;
    req_key = req_key + 1;

    // Insert entry
    let delete_tag_query = format!("INSERT INTO tag_requests (req_key, img_id, operation, old_tag)
    VALUES ({req_key},{img_id},'delete','{tag_name}')"); // TODO: make it safer (from sql injection)
    let _delete_tag_statement = conn.execute(delete_tag_query)?;
    
    Ok(1) // TODO: handle errors
}
