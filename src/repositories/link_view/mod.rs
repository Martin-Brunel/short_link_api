use crate::models::link_view::LinkView;
use crate::{dto::link_view::LinkViewInsert, db_connect::establish_connection};
use crate::schema;
use diesel::{RunQueryDsl, QueryDsl};
use rocket::http::Status;
use diesel::ExpressionMethods;

pub fn create(new_link_view: LinkViewInsert) -> usize {
    use self::schema::link_view::dsl::*; 
    let mut c = establish_connection();
    let inserted = diesel::insert_into(link_view)
    .values(&new_link_view)
    .execute(&mut c)
    .expect("insertion impossible");
    inserted
}

pub fn find_by_link_id(link_table_id: i32) -> Result<Vec<LinkView>, Status> {
    use self::schema::link_view::dsl::*;
    let mut c = establish_connection();


    match link_view
        .filter(link_id.eq(link_table_id))
        .filter(is_deleted.eq(0))
        .order_by(created_at.desc())
        .get_results::<LinkView>(&mut c) {
            Ok(res) => Ok(res),
            Err(_) => Err(Status::BadRequest)
        }
}