use std::env;

use diesel::PgConnection;
use diesel::prelude::*;

use crate::domain::{Device, InsertDevice};
use crate::schema::devices;

embed_migrations!("migrations");

pub fn migrate() {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
        .expect("Migration failed!");
}

pub fn get_devices(conn: &PgConnection) -> Vec<Device> {
    return devices::dsl::devices
        .load::<Device>(conn)
        .expect("Error loading all devices");
}

pub fn create_device(conn: &PgConnection, new_device: InsertDevice) -> Device {
    let result = diesel::insert_into(devices::table)
        .values(new_device)
        .get_result(conn) as QueryResult<Device>;
    return result.expect("Error saving new Node");
}

pub fn get_device_by_name(conn: &PgConnection, name: String) -> Option<Device> {
    return devices::dsl::devices
        .filter(devices::name.eq(&name))
        .first(conn)
        .optional()
        .expect(format!("Error loading device {}", name).as_str());
}
