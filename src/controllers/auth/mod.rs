use rocket::{serde::{ json::Json}, http::Status};
use crate::{dto::credentials::{Credentials}, managers, utils::jwt::create_jwt, dto::jwt::Jwt};

#[post("/login", format = "json", data="<credentials>")]
pub fn login(credentials: Json<Credentials>) -> Result<Json<Jwt>, Status> {
    let credentials = Credentials {
        email: credentials.email.clone(),
        password: credentials.password.clone(),
    };

    match managers::user::check_credentials(credentials) {
        Ok(user) => {
            let token = create_jwt(user);
            let jwt = Jwt {
                token
            };
            Ok(Json(jwt))
        },
        Err(e) => Err(e)
    }

}