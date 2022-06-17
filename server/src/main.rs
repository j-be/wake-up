#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;

use rocket::routes;
use rocket_cors::AllowedOrigins;

mod api_handler;
mod domain;
mod repo;
mod schema;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    repo::migrate();

    println!("Start");
    rocket::build()
        .mount("/wake-up/api/", routes![
                api_handler::awake,
                api_handler::get_devices,
                api_handler::post_device,
         ])
        .attach(api_handler::WakeUpDbConn::fairing())
        .attach(rocket_cors::CorsOptions::default()
            .allowed_origins(AllowedOrigins::some_exact(&[
                "http://localhost:8080",
                "https://owly.duckdns.org",
            ]))
            .to_cors()
            .expect("Error while configuring CORS"))
}
