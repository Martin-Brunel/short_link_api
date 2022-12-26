pub mod models;
pub mod schema;
pub mod db_connect;
pub mod repositories;
pub mod controllers;
pub mod managers;
pub mod dto;
pub mod utils;
pub mod guards;


#[macro_use] extern crate rocket;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/liste", routes![
                controllers::liste::get_all
            ])
        .mount("/user", routes![
                controllers::user::post_user
            ])
        .mount("/auth", routes![
            controllers::auth::login
        ])
}

