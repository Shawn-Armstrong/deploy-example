use crate::services::hello_service::hello_service;

pub async fn hello_get() -> axum::Json<crate::models::hello_state::HelloState> {
    hello_service().await
}
