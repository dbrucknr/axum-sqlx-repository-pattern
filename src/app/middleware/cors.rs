use tower_http::cors::{Any, CorsLayer};
// https://docs.rs/tower-http/latest/tower_http/cors/

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_credentials(true)
}
