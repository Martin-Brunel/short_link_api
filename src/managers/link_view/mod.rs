use rocket::http::Status;

use crate::{repositories, dto::link_view::LinkViewInsert};

pub fn insert(ip: String, link_id: i32) -> Result<bool, Status> {
    let new_link_view = LinkViewInsert {
        ip,
        link_id,
    };
    match repositories::link_view::create(new_link_view) {
        1 => Ok(true),
        _ => Err(Status::BadRequest)
    }
}