use axum::{Router, routing::get};

use super::Device;
use crate::app::devices::controller::DeviceControllers;

pub trait DeviceRoutes {
    fn routes() -> Router;
}

impl DeviceRoutes for Device {
    fn routes() -> Router {
        Router::new().route("/", get(Device::list))
    }
}
