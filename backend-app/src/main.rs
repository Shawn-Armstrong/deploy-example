mod models;
mod routes;
mod services;

use routes::create_routes;
use tokio::net::TcpListener;
use axum::serve;
use tracing_subscriber;
use sqlx::{PgPool, postgres::PgPoolOptions, Row};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .json()
        .init();

    let db_url = "postgres://admin:admin@postgres:5432/ucsc";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    // Run a simple query to test the connection
    let row = sqlx::query("SELECT message FROM greetings LIMIT 1")
        .fetch_one(&pool)
        .await?;

    let greeting: String = row.get("message");
    tracing::info!(greeting = %greeting, "Fetched greeting from database");

    let app = create_routes(pool);
    let address = "0.0.0.0:5000";
    tracing::info!("Starting server on {}", address);

    let listener = TcpListener::bind(address).await?;
    serve(listener, app).await?;
    Ok(())
}
