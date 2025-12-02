use crate::app::devices::{
    errors::DeviceServiceError,
    model::Device,
    repository::{DeviceRepository, DeviceRepositoryTrait},
    schemas::CreateDeviceRequestBody,
};

pub trait DeviceServiceTrait {
    fn get_all_devices(&self) -> impl Future<Output = Result<Vec<Device>, DeviceServiceError>>;
    fn create_device(
        &self,
        payload: CreateDeviceRequestBody,
    ) -> impl Future<Output = Result<Device, DeviceServiceError>>;
}

#[derive(Debug)]
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
