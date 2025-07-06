mod models;
mod routes;
mod services;

use routes::create_routes;
use tokio::net::TcpListener;
use axum::serve;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Set up structured JSON logging
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .json()
        .init();

    let app = create_routes();
    let address = "0.0.0.0:5000";
    tracing::info!("Starting server on {}", address);

    let listener = TcpListener::bind(address).await.unwrap();
    serve(listener, app).await.unwrap();
}
