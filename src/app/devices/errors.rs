use crate::app::devices::schemas::ApiErrorResponse;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::Error as SqlxError;
use tracing::{error, warn};

pub enum DeviceServiceError {
    RecordNotFound,
    ConnectionError,
    DatabaseError(sqlx::Error),
}
impl From<SqlxError> for DeviceServiceError {
    fn from(error: SqlxError) -> Self {
        match error {
            // No Row Found
            SqlxError::RowNotFound => {
                warn!("Sqlx: Record not found.");
                DeviceServiceError::RecordNotFound
            }
            // Connection Issues
            SqlxError::PoolTimedOut | SqlxError::PoolClosed | SqlxError::Io(_) => {
                warn!("Sqlx: Connection error: {:?}", error);
                DeviceServiceError::ConnectionError
            }
            // Database Constraints + SQL Syntax Errors
            _ => {
                error!("Sqlx: Unexpected Database error: {:?}", error);
                DeviceServiceError::DatabaseError(error)
            }
        }
    }
}

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
