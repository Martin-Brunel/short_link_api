use rocket::http::Status;
use rocket::response::Redirect;
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

#[get("/<code>")]
pub fn redirect_url(code: String) -> Result<Redirect, Status> {
    println!("{}", code);
    match managers::link::get_link_by_code(code) {
        Ok(link_output) => {
            let url = link_output.url;
            Ok(Redirect::to(url))
        },
        Err(status) => Err(status)
    }
}