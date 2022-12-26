use rocket::http::Status;
use rocket::serde::{ json::Json};
use crate::managers;
use crate::{ models::user::User, dto::user::UserInput };
use bcrypt::{DEFAULT_COST, hash};


#[post("/",  format = "json", data="<user_input>")]
pub fn post_user(user_input: Json<UserInput>) -> Result<Json<User>, Status> {
    let struct_user = UserInput {
        email: user_input.email.clone(),
        password: hash(user_input.password.clone(), DEFAULT_COST).expect("encrypt error"),
        roles: String::from("[\"ROLE_USER\"]")
    };
    match managers::user::create(struct_user) {
        Ok(new_user) => Ok(Json(new_user)),
        Err(_) => Err(Status { code: 400 })
    }
}