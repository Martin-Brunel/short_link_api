use chrono::NaiveDateTime;
use diesel::{self};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Queryable, Clone)]
#[diesel(table_name = link)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Brand))]
pub struct Link {
    pub id: i32,
    pub url: String,
    pub code: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub is_deleted: i8,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub brand_id: i32,
    pub label: String,
    pub nb_clicks: i32,
}