mod models;
mod routes;
mod services;
mod repository;
mod app_state;

use crate::app_state::AppState;
use routes::create_routes;
use tokio::net::TcpListener;
use axum::serve;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber;
use std::sync::Arc;

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

    let state = AppState { db: Arc::new(pool) };

    let app = create_routes(state.clone());
    let address = "0.0.0.0:5000";
    tracing::info!("Starting server on {}", address);

    let listener = TcpListener::bind(address).await?;
    serve(listener, app).await?;
    Ok(())
}
