use crate::{dto::link::{LinkInput, LinkOutput, LinkInsert}, repositories, utils::code::generate_code, models::user::User,};
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

pub fn get_link_by_code(code: String) -> Result<LinkOutput, Status> {
    match repositories::link::get_link_by_code(code) {
        Ok(link) => Ok(LinkOutput {
            url: link.url
        }),
        Err(status) => Err(status)
    }
}