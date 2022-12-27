use rocket::http::Status;
use rocket::serde::{ json::Json};
use crate::dto::link::{LinkInput, LinkOutput};
use crate::guards::security::Security;
use crate::managers;


#[post("/",  format = "json", data="<link_input>")]
pub fn post_link(authorized: Security, link_input: Json<LinkInput>) -> Result<Json<LinkOutput>, Status> {
    let user_id = authorized.user.id.clone();
    let link_input = LinkInput {
        url: link_input.url.clone()
    };
    match managers::link::create(link_input, user_id) {
        Ok(new_link) => Ok(Json(new_link)),
        Err(_) => Err(Status { code: 400 })
    }
}