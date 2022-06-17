use crate::schema::devices;

#[derive(Identifiable, Queryable, Serialize, Debug)]
pub struct Device {
    pub id: i32,
    pub name: String,
    pub mac: String,
}

#[derive(Insertable, Deserialize)]
#[table_name="devices"]
pub struct InsertDevice<'a> {
    pub name: &'a str,
    pub mac: &'a str,
}
