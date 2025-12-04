pub mod authentication;
pub mod devices;
pub mod middleware;

use authentication::AuthenticationModule;
use devices::DeviceModule;
use middleware::{compression::compress_responses, tracer::trace_request_response_cycle};

use axum::{
    Json, Router,
    http::{StatusCode, Uri},
};
use serde::Serialize;
use sqlx::postgres::PgPool;
use tracing::instrument;

// Consider relocating the fallback context
#[derive(Serialize)]
struct NotFoundResponse {
    message: String,
    path: String,
}
impl NotFoundResponse {
    fn new(message: &str, path: Uri) -> Self {
        Self {
            message: message.to_string(),
            path: path.to_string(),
        }
    }
}

#[instrument]
async fn fallback(uri: Uri) -> (StatusCode, Json<NotFoundResponse>) {
    (
        StatusCode::NOT_FOUND,
        Json(NotFoundResponse::new("Not Found", uri)),
    )
}

pub struct NetFx {
    devices: DeviceModule,
    authentication: AuthenticationModule,
    pub router: Router,
}
impl NetFx {
    pub fn new(pool: PgPool) -> Self {
        Self {
            // Base Router
            router: Router::new(),
            // Pass a memory reference of the pool to the DeviceModule
            devices: DeviceModule::new(&pool),
            authentication: AuthenticationModule::new(),
        }
    }

    pub fn api(self) -> Router {
        self.router
            .fallback(fallback)
            .nest(
                "/api",
                Router::new()
                    .nest("/devices", self.devices.api())
                    .nest("/auth", self.authentication.api()),
            )
            .layer(trace_request_response_cycle())
            .layer(compress_responses())
    }
}

pub struct TestApp;
