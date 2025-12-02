#[derive(Debug)]
pub struct Device {
    pub id: i32,
    pub serial_number: String,
}

impl Device {
    pub fn new(id: i32, serial_number: &str) -> Self {
        Self {
            id,
            serial_number: serial_number.to_string(),
        }
    }
}
