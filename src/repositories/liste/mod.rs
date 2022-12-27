use crate::db_connect::{establish_connection};
use crate::schema;
use crate::models::liste::Liste;
use diesel::prelude::*;


pub fn get_all() -> Vec<Liste> {
    use self::schema::liste::dsl::*; 
    let mut c = establish_connection();
    let results = liste
        .load::<Liste>(&mut c)
        .expect("Impossible de charger la liste");
    results
}