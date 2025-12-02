use axum::{Extension, Json};
use std::sync::Arc;

use super::DeviceModule;
use crate::app::devices::{
    schemas::ListDevicesResponse,
    service::{DeviceService, DeviceServiceTrait},
};

pub trait DeviceControllers {
    fn list(
        service: Extension<Arc<DeviceService>>,
    ) -> impl Future<Output = Json<ListDevicesResponse>>;
}

impl DeviceControllers for DeviceModule {
    async fn list(Extension(service): Extension<Arc<DeviceService>>) -> Json<ListDevicesResponse> {
        // TODO: Convert to a Result<Json<ListDevicesResponse>, Error> and begin to map Service layer errors into Controller layer errors
        let result = service.get_all_devices().await;
        Json(ListDevicesResponse { devices: result })
    }
}
