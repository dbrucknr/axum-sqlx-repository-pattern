use serde::{Deserialize, Serialize};

use crate::app::devices::model::Device;

#[derive(Serialize)]
pub struct ListDevicesResponse {
    pub devices: Vec<Device>,
}

#[derive(Deserialize)]
pub struct CreateDeviceRequestBody {
    pub serial_number: String,
}
