use diesel::prelude::*;
use diesel;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use crate::schema::user;


#[derive(Queryable, Deserialize, Serialize, Insertable, ToSchema)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub roles: String,
}