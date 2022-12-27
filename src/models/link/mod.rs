use chrono::NaiveDateTime;
use diesel::{self};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Queryable)]
#[diesel(table_name = link)]
#[diesel(belongs_to(User))]
pub struct Link {
    pub id: i32,
    pub url: String,
    pub code: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub is_deleted: i8,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}