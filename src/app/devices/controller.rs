use axum::{Extension, Json};
use std::sync::Arc;
use tracing::instrument;

use super::DeviceModule;
use crate::app::devices::{
    errors::DeviceControllerError,
    schemas::{CreateDeviceRequestBody, CreateDeviceResponse, ListDevicesResponse},
    service::{DeviceService, DeviceServiceTrait},
};

pub trait DeviceController {
    fn list(
        service: Extension<Arc<DeviceService>>,
    ) -> impl Future<Output = Result<Json<ListDevicesResponse>, DeviceControllerError>>;
    fn create(
        service: Extension<Arc<DeviceService>>,
        payload: Json<CreateDeviceRequestBody>,
    ) -> impl Future<Output = Result<Json<CreateDeviceResponse>, DeviceControllerError>>;
}

impl DeviceController for DeviceModule {
    #[instrument(level = "debug")]
    async fn list(
        Extension(service): Extension<Arc<DeviceService>>,
    ) -> Result<Json<ListDevicesResponse>, DeviceControllerError> {
        let devices = service.get_all_devices().await?;
        Ok(Json(ListDevicesResponse::new(devices)))
    }

    #[instrument(level = "debug", fields(payload = payload.serial_number))]
    async fn create(
        Extension(service): Extension<Arc<DeviceService>>,
        Json(payload): Json<CreateDeviceRequestBody>,
    ) -> Result<Json<CreateDeviceResponse>, DeviceControllerError> {
        let device = service.create_device(payload).await?;

        Ok(Json(CreateDeviceResponse::new(
            device,
            "Device created successfully".to_string(),
        )))
    }
}
