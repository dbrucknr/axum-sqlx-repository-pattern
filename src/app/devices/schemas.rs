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

#[derive(Serialize)]
pub struct CreateDeviceResponse {
    pub device: Device,
    pub message: String,
}

#[derive(Deserialize)]
pub struct CreateDeviceRequestBody {
    pub serial_number: String,
}
