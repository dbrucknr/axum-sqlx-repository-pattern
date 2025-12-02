use axum::{Extension, Router, routing::get};
use sqlx::SqlitePool;
use std::sync::Arc;

pub mod controller;
pub mod model;
pub mod repository;
pub mod routes;
pub mod service;

use crate::app::devices::controller::DeviceControllers;
use crate::app::devices::repository::DeviceRepository;
use crate::app::devices::service::DeviceService;

pub struct DeviceModule {
    service: DeviceService, // Attach as an extension to the router
    router: Router,
}
impl DeviceModule {
    pub fn new(pool: &SqlitePool) -> Self {
        // Add a repository (with a cloned sqlx pool) as a owned instance to the service
        let repository = DeviceRepository::new(&pool.clone());
        let service = DeviceService::new(repository);

        Self {
            service,
            router: Router::new(),
        }
    }
    pub fn api(self) -> Router {
        self.router
            .route("/", get(Self::list))
            .layer(Extension(Arc::new(self.service)))
    }
}
