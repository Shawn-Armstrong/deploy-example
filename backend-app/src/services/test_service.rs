use axum::Json;
use crate::models::test_state::TestState;
use tracing::{info, span, Level};

pub async fn hello_service() -> Json<TestState> {
    let span = span!(Level::INFO, "hello_service");
    let _guard = span.enter();

    let response = TestState {
        name: "My Name".to_string(),
        id: uuid::Uuid::new_v4().to_string(),
        message: "From the service layer".to_string(),
    };

    info!(id = %response.id, name = %response.name, "Generated test response");
    Json(response)
}
