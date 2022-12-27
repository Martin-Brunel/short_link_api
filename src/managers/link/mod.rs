use crate::{dto::link::{LinkInput, LinkOutput, LinkInsert}, repositories, utils::code::generate_code,};
use rocket::http::Status;
use dotenvy::dotenv;
use std::env;


pub fn create(link_input: LinkInput, user_id: i32) -> Result<LinkOutput, Status> {
    let password = generate_code();
    let link_insert = LinkInsert {
        code: password,
        url: link_input.url.clone(),
        user_id: user_id.clone()
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