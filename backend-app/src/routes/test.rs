use crate::services::test_service::hello_service;

pub async fn test_get() -> axum::Json<crate::models::test_state::TestState> {
    hello_service().await
}
