pub mod devices;

use crate::app::devices::{Device, routes::DeviceRoutes};

use axum::{Extension, Router};
use sqlx::SqlitePool;

pub struct NetFx {
    pub router: Router,
}
impl NetFx {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            // Base Router
            router: Router::new().layer(Extension(pool)),
        }
    }

    pub fn api(self) -> Router {
        self.router
            .nest("/api", Router::new().nest("/devices", Device::routes()))
    }
}

pub struct TestApp;
