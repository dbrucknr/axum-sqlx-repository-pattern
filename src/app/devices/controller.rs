use super::DeviceModule;
use crate::app::devices::service::{DeviceService, DeviceServiceTrait};
use axum::Extension;
use std::sync::Arc;

pub trait DeviceControllers {
    fn list(service: Extension<Arc<DeviceService>>) -> impl Future<Output = String>;
}

impl DeviceControllers for DeviceModule {
    async fn list(Extension(service): Extension<Arc<DeviceService>>) -> String {
        let result = service.get_all_devices().await;
        println!("Devices: {:?}", result);
        "Hello, World!".to_string()
    }
}
