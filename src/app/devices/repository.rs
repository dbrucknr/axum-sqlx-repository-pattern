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
        if let Ok(result) = sqlx::query_as!(Device, "SELECT id, serial_number FROM devices")
            .fetch_all(&self.pool)
            .await
        {
            result
        } else {
            Vec::new()
        }
    }
}
