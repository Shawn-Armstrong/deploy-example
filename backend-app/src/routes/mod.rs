use axum::{Router, routing::get};
use crate::app_state::AppState;

mod hello;

use hello::hello_get;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/api/hello", get(hello_get))
        .with_state(state)
}
