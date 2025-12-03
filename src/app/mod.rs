pub mod authentication;
pub mod devices;
pub mod middleware;

use authentication::AuthenticationModule;
use devices::DeviceModule;
use middleware::tracer::trace_request_response_cycle;

use axum::Router;

use sqlx::postgres::PgPool;

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
            .nest(
                "/api",
                Router::new()
                    .nest("/devices", self.devices.api())
                    .nest("/auth", self.authentication.api()),
            )
            .route_layer(trace_request_response_cycle())
    }
}

pub struct TestApp;
