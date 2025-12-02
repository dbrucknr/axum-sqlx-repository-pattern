use axum::{Router, routing::get};

use super::DeviceModule;
use crate::app::devices::controller::DeviceControllers;

pub trait DeviceRoutes {
    fn routes() -> Router;
}

impl DeviceRoutes for DeviceModule {
    fn routes() -> Router {
        Router::new().route("/", get(DeviceModule::list))
    }
}
