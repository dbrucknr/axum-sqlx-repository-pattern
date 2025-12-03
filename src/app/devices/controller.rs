use axum::{Extension, Json};
use std::sync::Arc;
use tracing::instrument;

use super::DeviceModule;
use crate::app::devices::{
    errors::DeviceControllerError,
    schemas::{CreateDeviceRequestBody, CreateDeviceResponse, ListDevicesResponse},
    service::{DeviceService, DeviceServiceTrait},
};

// Type Alias for DeviceController Response's
// - helps standardize JSON responses, and shorten type references
type ControllerResponse<T> = Result<Json<T>, DeviceControllerError>;

// Controller Trait (Interface)
// - provides the expected contact for which routes a controller will provide to the Device domain router
pub trait DeviceController {
    fn list(
        service: Extension<Arc<DeviceService>>,
    ) -> impl Future<Output = ControllerResponse<ListDevicesResponse>>;
    fn create(
        service: Extension<Arc<DeviceService>>,
        payload: Json<CreateDeviceRequestBody>,
    ) -> impl Future<Output = ControllerResponse<CreateDeviceResponse>>;
}

impl DeviceController for DeviceModule {
    #[instrument(level = "debug")]
    async fn list(
        Extension(service): Extension<Arc<DeviceService>>,
    ) -> ControllerResponse<ListDevicesResponse> {
        let devices = service.get_all_devices().await?;
        Ok(Json(ListDevicesResponse::new(devices)))
    }

    #[instrument(level = "debug", fields(payload = payload.serial_number))]
    async fn create(
        Extension(service): Extension<Arc<DeviceService>>,
        Json(payload): Json<CreateDeviceRequestBody>,
    ) -> ControllerResponse<CreateDeviceResponse> {
        let device = service.create_device(payload).await?;

        Ok(Json(CreateDeviceResponse::new(
            device,
            "Device created successfully".to_string(),
        )))
    }
}
