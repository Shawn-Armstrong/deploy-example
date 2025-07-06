pub mod hello;

use axum::{Router, routing::get};
use self::hello::hello_get;

pub fn create_routes() -> Router {
    Router::new().route("/api/hello", get(hello_get))
}
