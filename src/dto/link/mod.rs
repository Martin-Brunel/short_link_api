use diesel::{Insertable, Queryable};
use diesel;
use serde::{Serialize, Deserialize};
use crate::schema::link;

#[derive(Deserialize, Serialize, Debug)]
pub struct LinkInput {
    pub url: String,
    pub label: String,
}

#[derive(Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = link)]
pub struct LinkInsert {
    pub code: String,
    pub url: String,
    pub user_id: i32,
    pub brand_id: i32,
    pub label: String,
}

#[derive(Deserialize, Serialize)]
pub struct LinkOutput {
    pub url: String,
}