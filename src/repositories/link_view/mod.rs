use crate::{dto::link_view::LinkViewInsert, db_connect::establish_connection};
use crate::schema;
use diesel::{RunQueryDsl};

pub fn create(new_link_view: LinkViewInsert) -> usize {
    use self::schema::link_view::dsl::*; 
    let mut c = establish_connection();
    let inserted = diesel::insert_into(link_view)
    .values(&new_link_view)
    .execute(&mut c)
    .expect("insertion impossible");
    inserted
}