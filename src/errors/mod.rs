use rocket::{Request, serde::json::Json};

use crate::guards::security::{Security, ErrorObject};

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