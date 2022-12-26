use rocket::http::Status;

use crate::{repositories::liste, models::liste::Liste};


pub fn get_all() -> Result<Vec<Liste>, Status> {
    let test = liste::get_all();
    Ok(test)
}