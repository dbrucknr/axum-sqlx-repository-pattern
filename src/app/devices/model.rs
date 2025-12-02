#[derive(Debug)]
pub struct Device {
    id: i32,
    serial_number: String,
}

impl Device {
    pub fn new(id: i32, serial_number: &str) -> Self {
        Self {
            id,
            serial_number: serial_number.to_string(),
        }
    }
}
