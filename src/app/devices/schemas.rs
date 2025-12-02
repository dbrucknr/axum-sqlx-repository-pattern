use serde::{Deserialize, Serialize};

use crate::app::devices::model::Device;

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub error: String,
    pub message: String,
}
impl ApiErrorResponse {
    pub fn new(error: String, message: String) -> Self {
        Self { error, message }
    }
}

#[derive(Serialize)]
pub struct ListDevicesResponse {
    pub devices: Vec<Device>,
}
impl ListDevicesResponse {
    pub fn new(devices: Vec<Device>) -> Self {
        Self { devices }
    }
}

#[derive(Serialize)]
pub struct CreateDeviceResponse {
    pub device: Device,
    pub message: String,
}
impl CreateDeviceResponse {
    pub fn new(device: Device, message: String) -> Self {
        Self { device, message }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateDeviceRequestBody {
    pub serial_number: String,
}
