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

use rocket::{fairing::{Fairing, Info, Kind}, http::Header, Request, Response};
use rocket_dyn_templates::Template;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::{ OpenApi };
extern crate rocket_cors;
pub struct CORS;

#[macro_use] extern crate rocket;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Liste", description = "Endpoint relative to liste Object (test)")
    ),
)]
struct ApiDoc;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}




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
        .attach(CORS)
}

