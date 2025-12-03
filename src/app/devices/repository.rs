use sqlx::{Error as SqlxError, PgPool};

use crate::app::devices::{model::Device, schemas::CreateDeviceRequestBody};

pub trait DeviceRepositoryTrait {
    fn query_devices(&self) -> impl Future<Output = Result<Vec<Device>, SqlxError>>;
    fn create_device(
        &self,
        payload: CreateDeviceRequestBody,
    ) -> impl Future<Output = Result<Device, SqlxError>>;
}

#[derive(Debug)]
pub struct DeviceRepository {
    pool: PgPool,
}

impl DeviceRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

impl DeviceRepositoryTrait for DeviceRepository {
    async fn query_devices(&self) -> Result<Vec<Device>, SqlxError> {
        let devices = sqlx::query_as!(Device, "SELECT id, serial_number FROM devices")
            .fetch_all(&self.pool)
            .await?;

        Ok(devices)
    }

    async fn create_device(&self, payload: CreateDeviceRequestBody) -> Result<Device, SqlxError> {
        let created_device = sqlx::query_as!(
            Device,
            "INSERT INTO devices (serial_number) VALUES ($1) RETURNING id, serial_number",
            payload.serial_number
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created_device)
    }
}
