use rocket::http::Status;

use crate::{repositories, dto::link_view::LinkViewInsert, models::link_view::LinkView};

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

pub fn get_view_by_link(link_id: String, user_id: i32) -> Result<Vec<LinkView>, Status> {
    let link_id =  link_id.parse::<i32>().unwrap();
    match repositories::link::get_user_link_by_id(link_id, user_id) {
        Ok(link) => {
            match repositories::link_view::find_by_link_id(link.id) {
                Ok(links) => Ok(links),
                Err(status) => Err(status)
            }
        },
        Err(status) => Err(status)
    }

}