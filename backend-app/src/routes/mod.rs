pub mod test;

use axum::{Router, routing::get};
use self::test::test_get;

pub fn create_routes() -> Router {
    Router::new().route("/api/test", get(test_get))
}
