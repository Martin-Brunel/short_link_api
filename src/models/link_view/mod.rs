use chrono::NaiveDateTime;
use diesel::{self};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Queryable)]
#[diesel(table_name = link_view)]
#[diesel(belongs_to(Link))]
pub struct LinkView {
    pub id: i32,
    pub ip: String,
    pub created_at: NaiveDateTime,
    pub is_deleted: i8,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub link_id: i32
}