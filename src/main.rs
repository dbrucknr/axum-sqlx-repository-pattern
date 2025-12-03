pub mod app;
pub mod errors;
pub mod logs;

use axum::serve;
use dotenvy::dotenv;

use sqlx::postgres::PgPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

use app::NetFx;
use logs::logger;

#[tokio::main]
async fn main() -> errors::Returns<()> {
    dotenv().ok();
    logger();

    let url = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&url).await?;
    info!("Database connected");
    sqlx::migrate!().run(&pool).await?;
    info!("Database migrations applied");

    let netfx = NetFx::new(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on: http://{}", addr);

    serve(listener, netfx.api()).await?;

    Ok(())
}
