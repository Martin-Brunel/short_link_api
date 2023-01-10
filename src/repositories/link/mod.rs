use crate::db_connect::{establish_connection};
use crate::dto::link::{LinkInsert};
use crate::models::link::Link;
use crate::schema;
use chrono::Utc;
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

pub fn get_user_links(userid: i32) -> Result<Vec<Link>, Status> {
    use self::schema::link::dsl::*;
    let mut c = establish_connection();
    match link
        .filter(user_id.eq(userid))
        .filter(is_deleted.eq(0))
        .order_by(created_at.desc())
        .get_results::<Link>(&mut c) {
            Ok(links) => Ok(links),
            Err(_) => Err(Status::NotFound)
        }     
}

pub fn add_click(current_link: Link) -> Result<usize, Status> {
    use self::schema::link::dsl::*;
    let mut c = establish_connection();
    match diesel::update(link)
        .set((
            nb_clicks.eq(nb_clicks + 1),
            updated_at.eq(Utc::now().naive_utc())
        ))
        .filter(id.eq(current_link.id))
        .filter(is_deleted.eq(0))
        .execute(&mut c) {
            Ok(links) => Ok(links),
            Err(_) => Err(Status::NotFound)
        } 
}

pub fn get_link_by_id(link_id: i32) -> Result<Link, Status> {
    use self::schema::link::dsl::*;
    let mut c = establish_connection();
    match link
        .filter(id.eq(link_id))
        .filter(is_deleted.eq(0))
        .first::<Link>(&mut c) {
            Ok(link_line) => Ok(link_line),
            Err(_) => Err(Status::NotFound)
        }     
}

pub fn get_user_link_by_id(link_id: i32, user_from_id: i32) -> Result<Link, Status> {
    use self::schema::link::dsl::*;
    let mut c = establish_connection();
    match link
        .filter(id.eq(link_id))
        .filter(user_id.eq(user_from_id))
        .filter(is_deleted.eq(0))
        .first::<Link>(&mut c) {
            Ok(link_line) => Ok(link_line),
            Err(_) => Err(Status::NotFound)
        }     
}