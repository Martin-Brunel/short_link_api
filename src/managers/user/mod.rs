use crate::dto::credentials::{Credentials};
use crate::dto::user::UserInput;
use crate::models::user::User;
use crate::repositories::{user, self};
use bcrypt::{verify};
use rocket::http::Status;


pub fn create(input_user: UserInput) -> Result<User, Status> {
    let email = input_user.email.clone();
    user::create(input_user);
    match user::get_by_email(email) {
        Ok(user_object) => Ok(user_object),
        Err(e) => Err(e)
    }
} 

pub fn check_credentials(credentials: Credentials) -> Result<User, Status>  {
    match repositories::user::get_by_email(credentials.email) {
        Ok(user) => {
            let verif =  verify(credentials.password, &user.password).unwrap();
            match verif {
                true => Ok(user),
                _ => Err(Status::Forbidden)
            }
        },
        Err(e) => Err(e)
    }
}