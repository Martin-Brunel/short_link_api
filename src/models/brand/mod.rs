use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use crate::schema::brand;


#[derive(Queryable, Deserialize, Serialize, Insertable, ToSchema)]
#[table_name = "brand"]
pub struct Brand {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub is_deleted: i8,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}