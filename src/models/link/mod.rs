use diesel::{prelude::*};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Queryable)]
pub struct Link {
    id: i32,
    user_id: i32,
    url: String,
    code: String,
    is_deleted: bool,
    created_at: String,
    deleted_at: String,
    updated_at: String,
}