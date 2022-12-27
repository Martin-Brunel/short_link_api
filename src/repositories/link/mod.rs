use crate::db_connect::{establish_connection};
use crate::dto::link::{LinkInsert};
use crate::models::link::Link;
use crate::schema;
use rocket::http::Status;
use diesel::{RunQueryDsl, QueryDsl};
use diesel::ExpressionMethods;

pub fn create(new_link: LinkInsert) -> usize {
    use self::schema::link::dsl::*; 
    let mut c = establish_connection();
    let inserted = diesel::insert_into(link)
    .values(&new_link)
    .execute(&mut c)
    .expect("insertion impossible");
    inserted
}

pub fn get_link_by_code(link_code: String) -> Result<Link, Status> {
    use self::schema::link::dsl::*;
    let mut c = establish_connection();
    match link
        .filter(code.eq(link_code))
        .filter(is_deleted.eq(0))
        .first::<Link>(&mut c) {
            Ok(link_line) => Ok(link_line),
            Err(_) => Err(Status::NotFound)
        }     
}