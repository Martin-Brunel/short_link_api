use chrono::NaiveDateTime;
use diesel::{self};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Queryable, Clone)]
#[diesel(table_name = link_view)]
#[diesel(belongs_to(Link))]
pub struct LinkView {
    pub id: i32,
    pub ip: String,
    pub link_id: i32,
    pub created_at: NaiveDateTime,
    pub is_deleted: i8,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}