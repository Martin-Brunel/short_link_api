use rocket::{serde::{ json::Json}, http::Status,};
use crate::{ models::{ link_view::LinkView}, managers};
use crate::guards::security::Security;

#[get("/<id>")]
pub fn get_view_by_link(authorized: Security, id: String) -> Result<Json<Vec<LinkView>>, Status> {
    
    let user = authorized.user;
    match managers::link_view::get_view_by_link(id, user.id) {
        Ok(returning) => Ok(Json(returning)),
        Err(e) => Err(e)
    }
}