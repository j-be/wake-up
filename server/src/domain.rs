use crate::schema::devices;

#[derive(Identifiable, Queryable, Serialize, Debug)]
pub struct Device {
    pub id: i32,
    pub name: String,
    pub mac: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name="devices"]
pub struct InsertDevice {
    pub name: String,
    pub mac: String,
}
