use axum::{Router, routing::get};
use crate::services::hello_service::hello_service;
use sqlx::PgPool;

pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/api/hello", get(hello_service))
        .with_state(pool)
}