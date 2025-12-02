use crate::app::devices::{
    model::Device,
    repository::{DeviceRepository, DeviceRepositoryTrait},
};

pub trait DeviceServiceTrait {
    fn get_all_devices(&self) -> impl Future<Output = Vec<Device>>;
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
    async fn get_all_devices(&self) -> Vec<Device> {
        let result = self.repository.query_devices().await;
        // Service logic here (e.g., filtering, sorting, etc.)
        result
    }
}
