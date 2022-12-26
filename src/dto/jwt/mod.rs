use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Jwt {
    pub token: String,
}