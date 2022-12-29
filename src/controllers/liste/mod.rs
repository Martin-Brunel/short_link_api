use rocket::{serde::{ json::Json}, http::Status,};
use crate::{ models::{liste::Liste}, managers};
use crate::guards::security::Security;

#[get("/")]
pub fn get_all(_authorized: Security) -> Result<Json<Vec<Liste>>, Status> {
    match managers::liste::get_all() {
        Ok(returning) => Ok(Json(returning)),
        Err(e) => Err(e)
    }
}