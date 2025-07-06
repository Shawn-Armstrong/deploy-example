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
    use axum::Json;

    fn init_tracing() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("info")
            .with_test_writer() // directs output to test logs
            .try_init(); // prevent panicking if already initialized
    }

    #[tokio::test]
    async fn test_hello_service_response_structure() {
        let result = hello_service().await;
        let data = result.0;

        assert_eq!(data.name, "My Name");
        assert_eq!(data.message, "From the service layer");
        assert!(!data.id.is_empty());
    }
}
