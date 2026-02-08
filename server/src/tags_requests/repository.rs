use anyhow;
use sqlite;
use anyhow::Result;
use crate::tags_requests::model::PendingTagResponse;

pub fn get_all_pending_tags () -> Result<Vec<PendingTagResponse>, anyhow::Error> {
    let conn = sqlite::open("./database/labelsys.db")?; // drop method is called implicitly

    let all_tags_query = "SELECT * FROM tag_requests LEFT JOIN images ON tag_requests.img_id = images.id LIMIT 5";
    let mut all_tags_statement = conn.prepare(all_tags_query)?;

    let mut found_requests: Vec<PendingTagResponse> = Vec::new();
    
    while let sqlite::State::Row = all_tags_statement.next()? {
        let req_key_i64: i64 = all_tags_statement.read(0)?;
        let req_key = req_key_i64 as u32;
        let operation: String = all_tags_statement.read(2)?;
        let old_tag_opt: Option<String> = all_tags_statement.read(3)?;
        let new_tag_opt: Option<String> = all_tags_statement.read(4)?;
        let img_url = all_tags_statement.read(6)?;

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
    let conn = sqlite::open("./database/labelsys.db")?; // drop method is called implicitly

    let remove_request_query = format!("DELETE FROM tag_requests WHERE req_key={req_key}");
    let mut _remove_request_statement = conn.execute(remove_request_query)?;

    Ok(true) // TODO: handle errors
}

pub fn approve_tag_request(req_key: u32) -> Result<bool> {
    // Scope: in order to close the connection before calling remove_tag_request(),
    // otherwise the connection to the db is locked
    {
        let conn = sqlite::open("./database/labelsys.db")?; // drop method is called implicitly

        let tag_query = format!("SELECT * FROM tag_requests WHERE req_key={req_key}");
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
            set_new_tag(img_id, new_tag.clone(), &conn);
        }
        if cur_req_type == "edit" {
            set_new_tag(img_id, new_tag.clone(), &conn);
            delete_tag(img_id, old_tag.clone(), &conn);
        }
        if cur_req_type == "delete" {
            delete_tag(img_id, old_tag.clone(), &conn); // TODO: remove cloning?
        }
    }

    let _ = remove_tag_request(req_key);

    Ok(true) // TODO: handle errors
}

fn set_new_tag(image_id: u32, tag_name: String, conn: &sqlite::Connection) {
    let new_tag_query = format!("INSERT INTO tags (img_id, tag)
    VALUES ({image_id},'{tag_name}')"); // FIXME: make it safer (from sql injection)
    let _new_tag_statement = conn.execute(new_tag_query).unwrap(); // TODO: replace unwrap
}

fn delete_tag(image_id: u32, tag_name: String, conn: &sqlite::Connection) {
    let delete_tag_query = format!("DELETE FROM tags
    WHERE (img_id = {} AND tag = '{}')", image_id, tag_name);
    let _delete_tag_statement = conn.execute(delete_tag_query).unwrap(); // TODO: replace unwrap
}
