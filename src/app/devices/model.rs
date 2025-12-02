use serde::Serialize;

// TODO: Implement a From<Device> for DeviceModel
#[derive(Debug, Serialize)]
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
