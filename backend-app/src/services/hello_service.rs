use crate::app_state::AppState;
use crate::repository::greeting_repo;
use axum::Json;
use crate::models::hello_state::HelloState;
use uuid::Uuid;

pub async fn hello_service(state: AppState) -> anyhow::Result<Json<HelloState>> {
    let greeting = greeting_repo::get_greeting(&state.db).await?;

    let response = HelloState {
        name: "My Name".to_string(),
        id: Uuid::new_v4().to_string(),
        message: greeting,
    };

    Ok(Json(response))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use axum::Json;

//     fn init_tracing() {
//         let _ = tracing_subscriber::fmt()
//             .with_env_filter("info")
//             .with_test_writer() // directs output to test logs
//             .try_init(); // prevent panicking if already initialized
//     }

//     #[tokio::test]
//     async fn test_hello_service_response_structure() {
//         let result = hello_service().await;
//         let data = result.0;

//         assert_eq!(data.name, "My Name");
//         assert_eq!(data.message, "From the service layer");
//         assert!(!data.id.is_empty());
//     }
// }
