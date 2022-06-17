use wakey;
use std::net::SocketAddr;
use crate::domain::{Device, InsertDevice};
use crate::repo;
use rocket::serde::json::Json;
use rocket_sync_db_pools::{database, diesel};

#[database("wakeup_db")]
pub struct WakeUpDbConn(diesel::PgConnection);

#[get("/awake/<name>")]
pub async fn awake(conn: WakeUpDbConn, name: String) -> Json<Option<Device>> {
    match conn.run(move |c| repo::get_device_by_name(c, name)).await {
        Some(device) => {
            wakey::WolPacket::from_string(device.mac.as_str(), ':')
                .send_magic_to(SocketAddr::from(([0,0,0,0], 0)), SocketAddr::from(([255,255,255,255], 9)))
                .expect(format!("Error while sending magic packet for {:?}", device).as_str());
            Json(Some(device))
        },
        _ => Json(None)
    }
}

#[get("/devices")]
pub async fn get_devices(conn: WakeUpDbConn) -> Json<Vec<Device>> {
    return conn.run(|c| Json(repo::get_devices(c))).await;
}

#[post("/devices", data = "<new_device>")]
pub async fn post_device(conn: WakeUpDbConn, new_device: Json<InsertDevice>) -> Json<Device> {
    return conn.run(move |c| Json(repo::create_device(c, new_device.into_inner()))).await;
}
