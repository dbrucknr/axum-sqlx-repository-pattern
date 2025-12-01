pub mod app;
pub mod errors;

use axum::serve;
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> errors::Returns<()> {
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;

    let app = app::NetFx::new(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    let listener = TcpListener::bind(addr).await?;

    serve(listener, app.api()).await?;

    Ok(())
}
