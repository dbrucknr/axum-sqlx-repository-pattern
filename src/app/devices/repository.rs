use sqlx::SqlitePool;

use crate::app::devices::model::Device;

pub trait DeviceRepositoryTrait {
    fn query_devices(&self) -> impl Future<Output = Vec<Device>>;
}

pub struct DeviceRepository {
    pool: SqlitePool,
}

impl DeviceRepository {
    pub fn new(pool: &SqlitePool) -> Self {
        Self { pool: pool.clone() }
    }
}

impl DeviceRepositoryTrait for DeviceRepository {
    async fn query_devices(&self) -> Vec<Device> {
        // Use the pool to query the database here...
        vec![Device::new(1, "1234567890"), Device::new(2, "0987654321")]
    }
}
