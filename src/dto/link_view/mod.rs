use diesel::Insertable;
use serde::{Serialize, Deserialize};

use crate::schema::link_view;

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = link_view)]
pub struct LinkViewInsert {
    pub link_id: i32,
    pub ip: String,
}