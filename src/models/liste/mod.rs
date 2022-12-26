use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Queryable)]
pub struct Liste {
    pub id: i32,
    pub libelle: String,
}