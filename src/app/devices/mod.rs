use axum::{Extension, Router, routing::get};
use sqlx::SqlitePool;
use std::sync::Arc;

pub mod controller;
pub mod model;
pub mod repository;
pub mod schemas;
pub mod service;

use crate::app::devices::controller::DeviceControllers;
use crate::app::devices::repository::DeviceRepository;
use crate::app::devices::service::DeviceService;

// TODO: Add Schemas for request / response serialization + deserialization
// use serde::{Deserialize, Serialize};

pub struct DeviceModule {
    service: DeviceService,
    router: Router,
}
impl DeviceModule {
    pub fn new(pool: &SqlitePool) -> Self {
        // Build Module Dependencies
        let repository = DeviceRepository::new(&pool); // NOTE: I may need to clone the pool at this layer
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
