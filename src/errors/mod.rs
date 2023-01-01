use rocket::{Request, serde::json::Json};
use rocket_dyn_templates::{Template, context};
use serde::{Serialize, Deserialize};

use crate::guards::{security::{Security}, user_admin::UserAdmin};

#[derive(Debug, Serialize, Deserialize)]
pub struct  ErrorObject {
    pub message: String,
}

#[catch(401)]
pub async fn unauthorized(req: &Request<'_>) -> Json<ErrorObject> {
    let (_, todo_error) = req.guard::<Security>().await.failed().unwrap();

    Json(todo_error)
}

#[catch(403)]
pub async fn forbidden(req: &Request<'_>) -> Json<ErrorObject> {
    let (_, todo_error) = req.guard::<Security>().await.failed().unwrap();

    Json(todo_error)
}

#[catch(404)]
pub async fn notfound() -> Template {

    Template::render("notfound", context! {})
}