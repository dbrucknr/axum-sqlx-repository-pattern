use sqlx::Error as SqlxError;

use crate::app::devices::{
    model::Device,
    repository::{DeviceRepository, DeviceRepositoryTrait},
    schemas::CreateDeviceRequestBody,
};

// Catch Errors from the Repository Layer and Map them into Service Errors
pub enum DeviceServiceError {
    RecordNotFound,
    ConnectionError,
    DatabaseError(sqlx::Error),
}
impl From<SqlxError> for DeviceServiceError {
    fn from(error: SqlxError) -> Self {
        match error {
            // No Row Found
            SqlxError::RowNotFound => DeviceServiceError::RecordNotFound,
            // Connection Issues
            SqlxError::PoolTimedOut | SqlxError::PoolClosed | SqlxError::Io(_) => {
                DeviceServiceError::ConnectionError
            }
            // Database Constraints + SQL Syntax Errors
            _ => DeviceServiceError::DatabaseError(error),
        }
    }
}

pub trait DeviceServiceTrait {
    fn get_all_devices(&self) -> impl Future<Output = Result<Vec<Device>, DeviceServiceError>>;
    fn create_device(
        &self,
        payload: CreateDeviceRequestBody,
    ) -> impl Future<Output = Result<Device, DeviceServiceError>>;
}

pub struct DeviceService {
    repository: DeviceRepository,
}
impl DeviceService {
    pub fn new(repository: DeviceRepository) -> Self {
        Self { repository }
    }
}
impl DeviceServiceTrait for DeviceService {
    async fn get_all_devices(&self) -> Result<Vec<Device>, DeviceServiceError> {
        let result = self.repository.query_devices().await?;
        // Service logic here (e.g., filtering, sorting, etc.)
        Ok(result)
    }
    async fn create_device(
        &self,
        payload: CreateDeviceRequestBody,
    ) -> Result<Device, DeviceServiceError> {
        let result = self.repository.create_device(payload).await?;
        // Service logic here (e.g., filtering, sorting, etc.)
        Ok(result)
    }
}
