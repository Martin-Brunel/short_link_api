pub mod models;
pub mod schema;
pub mod db_connect;
pub mod repositories;
pub mod controllers;
pub mod managers;
pub mod dto;
pub mod utils;
pub mod guards;
pub mod errors;

use rocket_dyn_templates::Template;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::{ OpenApi };

#[macro_use] extern crate rocket;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Liste", description = "Endpoint relative to liste Object (test)")
    ),
)]
struct ApiDoc;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![errors::unauthorized, errors::forbidden, errors::notfound])
        .mount(
            "/",
            SwaggerUi::new("/doc/<_..>").url("/api-doc/openapi.json", ApiDoc::openapi()),
        )
        .mount("/", routes![controllers::link::redirect_url])
        .mount("/liste", routes![
                controllers::liste::get_all
            ])
        .mount("/user", routes![
                controllers::user::post_user
            ])
        .mount("/link", routes![
            controllers::link::post_link,
            controllers::link::get_links
        ])
        .mount("/auth", routes![
            controllers::auth::login
        ])
        .attach(Template::fairing())
}

