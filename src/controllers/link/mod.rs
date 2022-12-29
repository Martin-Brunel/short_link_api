use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::{ json::Json};
use rocket_client_addr::ClientRealAddr;
use crate::dto::link::{LinkInput, LinkOutput};
use crate::guards::security::Security;
use crate::managers;


#[post("/",  format = "json", data="<link_input>")]
pub fn post_link(authorized: Security, link_input: Json<LinkInput>) -> Result<Json<LinkOutput>, Status> {
    let user = authorized.user;
    let link_input = LinkInput {
        url: link_input.url.clone(),
        label: link_input.label.clone()
    };
    match managers::link::create(link_input, user) {
        Ok(new_link) => Ok(Json(new_link)),
        Err(_) => Err(Status { code: 400 })
    }
}

#[get("/<code>")]
pub fn redirect_url(socket_adress: ClientRealAddr ,code: String) -> Result<Redirect, Status> {
    
    match managers::link::get_link_by_code(code) {
        Ok(link_output) => {
            let ip = socket_adress.get_ipv4_string().unwrap(); 
            managers::link_view::insert(ip, link_output.id).unwrap();
            let url = link_output.url;
            Ok(Redirect::to(url))
        },
        Err(status) => Err(status)
    }
}