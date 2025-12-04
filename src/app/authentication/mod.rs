use axum::{Router, routing::get};

pub mod controller;

use controller::AuthenticationController;

pub struct AuthenticationModule {
    router: Router,
}

impl AuthenticationModule {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    pub fn api(self) -> Router {
        self.router
            .route("/login", get(Self::login))
            .route("/logout", get(Self::logout))
            .route("/authenticate", get(Self::authenticate))
            .route("/refresh", get(Self::refresh))
    }
}
