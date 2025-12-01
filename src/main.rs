pub mod app;
pub mod errors;
pub mod repo;

#[tokio::main]
async fn main() -> errors::Returns<()> {
    let app = app::App::new();

    Ok(())
}
