use axum::{
    Extension, Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

use super::DeviceModule;
use crate::app::devices::{
    schemas::{
        ApiErrorResponse, CreateDeviceRequestBody, CreateDeviceResponse, ListDevicesResponse,
    },
    service::{DeviceService, DeviceServiceError, DeviceServiceTrait},
};

// This enum may need to be tightened down a bit byt capturing more HTTP Response coodes and contexts
pub enum DeviceControllerError {
    NotFound,
    BadRequest,
    InternalServerError,
}
impl From<DeviceServiceError> for DeviceControllerError {
    fn from(error: DeviceServiceError) -> Self {
        match error {
            DeviceServiceError::RecordNotFound => DeviceControllerError::NotFound,
            DeviceServiceError::ConnectionError => DeviceControllerError::BadRequest,
            DeviceServiceError::DatabaseError(_) => DeviceControllerError::InternalServerError,
        }
    }
}
impl IntoResponse for DeviceControllerError {
    fn into_response(self) -> Response {
        let (status, error_response) = match self {
            DeviceControllerError::NotFound => (
                StatusCode::NOT_FOUND,
                ApiErrorResponse::new(
                    "Not Found".to_string(),
                    "The requested device was not found.".to_string(),
                ),
            ),
            DeviceControllerError::BadRequest => (
                StatusCode::BAD_REQUEST,
                ApiErrorResponse::new(
                    "Bad Request".to_string(),
                    "Invalid request parameters.".to_string(),
                ),
            ),
            DeviceControllerError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new(
                    "Internal_server_error".to_string(),
                    "Internal server error".to_string(),
                ),
            ),
        };
        (status, Json(error_response)).into_response()
    }
}

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
    async fn list(
        Extension(service): Extension<Arc<DeviceService>>,
    ) -> Result<Json<ListDevicesResponse>, DeviceControllerError> {
        // TODO: Convert to a Result<Json<ListDevicesResponse>, Error> and begin to map Service layer errors into Controller layer errors
        let devices = service.get_all_devices().await?;
        Ok(Json(ListDevicesResponse::new(devices)))
    }

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
