use axum::{
    Extension, Router,
    routing::{get, post},
};

use sqlx::postgres::PgPool;
use std::sync::Arc;

pub mod controller;
pub mod errors;
pub mod model;
pub mod repository;
pub mod schemas;
pub mod service;

use controller::DeviceController;
use repository::DeviceRepository;
use service::DeviceService;

pub struct DeviceModule {
    service: DeviceService,
    router: Router,
}
impl DeviceModule {
    pub fn new(pool: &PgPool) -> Self {
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
            .route("/", post(Self::create))
            .layer(Extension(Arc::new(self.service)))
    }
}
