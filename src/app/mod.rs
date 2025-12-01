use axum::Router;

pub struct App {
    pub router: Router,
}
impl App {
    pub async fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }
}

pub struct TestApp;
