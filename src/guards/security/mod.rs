use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, Validation, DecodingKey};
use rocket::{Request, request::{self, Outcome}, http::Status};
use dotenvy::dotenv;
use std::env;

use crate::{utils::jwt::JWT, repositories, models::user::User};
pub struct Security {
   pub user: User,
}

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for Security {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        dotenv().ok(); 
        let keys: Vec<_> = req.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((rocket::http::Status::Forbidden, ApiKeyError::Missing));
        }
        let jwt_secret = env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set");

        let jwt = keys[0];
        let jwt = jwt.replace("Bearer ", "");
        match decode::<JWT>(jwt.as_ref(), &DecodingKey::from_secret(jwt_secret.as_ref()), &Validation::new(Algorithm::HS256)) {
            Ok(token) => {
                match repositories::user::get_by_email(token.claims.email) {
                    Ok(user) => {
                        let security = Security {
                            user: user
                        };
                        Outcome::Success(security)
                    },
                    Err(_e) => Outcome::Failure((Status::Forbidden, ApiKeyError::Invalid)) 
                }
            },
            Err(_e) => {
                Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid))
            }
        }

    }
}