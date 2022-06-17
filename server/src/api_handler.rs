use rocket_contrib::json::Json;
use wakey;
use std::net::SocketAddr;
use crate::domain::{Device, InsertDevice};
use crate::repo;
use rocket::http::RawStr;

#[database("wakeup_db")]
pub struct WakeUpDbConn(diesel::PgConnection);

#[get("/awake/<name>")]
pub fn awake(conn: WakeUpDbConn, name: &RawStr) -> Json<Option<Device>> {
    match repo::get_device_by_name(&*conn, name) {
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
pub fn get_devices(conn: WakeUpDbConn) -> Json<Vec<Device>> {
    return Json(repo::get_devices(&*conn));
}

#[post("/devices", data = "<new_device>")]
pub fn post_device(conn: WakeUpDbConn, new_device: Json<InsertDevice>) -> Json<Device> {
    return Json(repo::create_device(&*conn, new_device.into_inner()));
}
