pub mod devices;

use crate::app::devices::DeviceModule;

use axum::Router;
use sqlx::SqlitePool;

pub struct NetFx {
    devices: DeviceModule,
    pub router: Router,
}
impl NetFx {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            // Base Router
            router: Router::new(),
            devices: DeviceModule::new(&pool),
        }
    }

    pub fn api(self) -> Router {
        self.router
            .nest("/api", Router::new().nest("/devices", self.devices.api()))
    }
}

pub struct TestApp;
