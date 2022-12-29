use crate::{dto::link::{LinkInput, LinkOutput, LinkInsert}, repositories, utils::code::generate_code, models::{user::User, link::Link}};
use rocket::http::Status;
use dotenvy::dotenv;
use std::env;


pub fn create(link_input: LinkInput, user: User) -> Result<LinkOutput, Status> {
    let password = generate_code();
    let link_insert = LinkInsert {
        code: password,
        url: link_input.url.clone(),
        user_id: user.id.clone(),
        brand_id: user.brand_id.clone(),
        label: link_input.label.clone()
    };
    let code = link_insert.code.clone();

    let insert = repositories::link::create(link_insert);
    match insert {
        1 => {
            dotenv().ok();
            let mut base_url = String::from(env::var("BASE_URL").unwrap()).to_owned();
            base_url.push_str(&code[..]);
            Ok(LinkOutput {
                url: base_url
            })
        },
        _ => Err(Status::InternalServerError),
    }
    
} 

pub fn get_link_by_code(code: String) -> Result<Link, Status> {
    match repositories::link::get_link_by_code(code) {
        Ok(link) => Ok(link),
        Err(status) => Err(status)
    }
}

pub fn get_user_links(user: User) -> Result<Vec<Link>, Status> {
    match repositories::link::get_user_links(user.id) {
        Ok(links) => Ok(links),
        Err(status) => Err(status)
    }
}

pub fn add_click(link: Link) -> Result<bool, Status> {
    match repositories::link::add_click(link) {
        Ok(updated) => {
            match updated {
                1 => Ok(true),
                _ => Err(Status::BadRequest)
            }
        },
        Err(status) => Err(status)
    }
}