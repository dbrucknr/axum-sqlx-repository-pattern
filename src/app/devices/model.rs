use serde::Serialize;
use sqlx::FromRow;

// TODO: Implement a From<Device> for DeviceModel
#[derive(Debug, Serialize, FromRow)]
pub struct Device {
    pub id: i64,
    pub serial_number: String,
}

impl Device {
    pub fn new(id: i64, serial_number: &str) -> Self {
        Self {
            id,
            serial_number: serial_number.to_string(),
        }
    }
}
