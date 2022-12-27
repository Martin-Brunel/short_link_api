use crate::db_connect::{establish_connection};
use crate::dto::link::{LinkInsert};
use crate::schema;
use diesel::{RunQueryDsl};

pub fn create(new_link: LinkInsert) -> usize {
    use self::schema::link::dsl::*; 
    let mut c = establish_connection();
    let inserted = diesel::insert_into(link)
    .values(&new_link)
    .execute(&mut c)
    .expect("insertion impossible");
    inserted
}
