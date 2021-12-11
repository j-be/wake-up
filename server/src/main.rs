#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;

use rocket::routes;

mod api_handler;
mod domain;
mod repo;
mod schema;

fn main() {
    dotenv().ok();

    repo::migrate();

    println!("Start");
    rocket::ignite()
        .mount("/wake-up/api/", routes![
                api_handler::awake,
                api_handler::get_devices,
                api_handler::post_device,
         ])
        .attach(api_handler::WakeUpDbConn::fairing())
        .launch();
}
