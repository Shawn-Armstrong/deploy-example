use axum::{
    routing::get,
    Json, Router,
    response::IntoResponse,
};
use serde::{Serialize, Deserialize};
use tracing::{info, error};
use tracing_subscriber::{fmt, EnvFilter};

#[derive(Serialize, Deserialize)]
struct TestState {
    name: String,
    id: String,
    message: String,
}

#[tokio::main]
async fn main() {
    // 🔧 Initialize structured JSON logger
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("debug".parse().unwrap()))
        .json()
        .init();

    info!("🚀 Starting backend server on http://0.0.0.0:5000");

    let app = Router::new().route("/api/test", get(test_get));

    let address = "0.0.0.0:5000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[axum::debug_handler]
async fn test_get() -> impl IntoResponse {
    let response = TestState {
        name: "Test Name".to_string(),
        id: uuid::Uuid::new_v4().to_string(),
        message: "Hello, World!".to_string(),
    };

    // ✍️ Log the response info
    info!(
        name = response.name,
        id = response.id,
        message = response.message,
        "Handled /api/test request"
    );

    Json(response)
}
