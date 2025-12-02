pub mod app;
pub mod errors;
pub mod logs;

use axum::serve;
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use app::NetFx;
use logs::logger;

#[tokio::main]
async fn main() -> errors::Returns<()> {
    dotenv().ok();
    logger();

    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;
    sqlx::migrate!().run(&pool).await?;

    let app = NetFx::new(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    let listener = TcpListener::bind(addr).await?;

    serve(listener, app.api()).await?;

    Ok(())
}
