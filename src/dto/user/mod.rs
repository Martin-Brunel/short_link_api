use diesel::{Insertable, Queryable};
use diesel;
use serde::{Serialize, Deserialize};
use crate::schema::user;

#[derive(Deserialize, Serialize, Insertable, Queryable)]
#[table_name = "user"]
pub struct UserInput {
    pub email: String,
    pub password: String,
    pub roles: String
}