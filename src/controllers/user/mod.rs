use rocket::http::Status;
use rocket::serde::{ json::Json};
use crate::dto::user::UserInsert;
use crate::managers;
use crate::{ models::user::User, dto::user::UserInput };
use bcrypt::{DEFAULT_COST, hash};


#[post("/",  format = "json", data="<user_input>")]
pub fn post_user(user_input: Json<UserInput>) -> Result<Json<User>, Status> {
    let struct_user = UserInsert {
        email: user_input.email.clone(),
        password: hash(user_input.password.clone(), DEFAULT_COST).expect("encrypt error"),
        roles: String::from("[\"ROLE_USER\"]"),
        brand_id: user_input.brand_id.clone()
    };
    match managers::user::create(struct_user) {
        Ok(new_user) => Ok(Json(new_user)),
        Err(_) => Err(Status { code: 400 })
    }
}