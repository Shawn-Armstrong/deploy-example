use axum::Json;
use crate::models::hello_state::HelloState;
use tracing::{info, span, Level};

pub async fn hello_service() -> Json<HelloState> {
    let span = span!(Level::INFO, "hello_service");
    let _guard = span.enter();

    let response = HelloState {
        name: "My Name".to_string(),
        id: uuid::Uuid::new_v4().to_string(),
        message: "From the service layer".to_string(),
    };

    info!(id = %response.id, name = %response.name, "Generated test response");
    Json(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hello_service() {
        let response = hello_service().await;
        assert_eq!(response.message, "From the service layer");
    }
}
