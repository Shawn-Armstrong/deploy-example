use axum::{extract::State, Json};
use crate::{app_state::AppState, models::hello_state::HelloState};
use crate::services::hello_service::hello_service;

pub async fn hello_get(State(state): State<AppState>) -> Json<HelloState> {
    hello_service(state)
        .await
        .unwrap_or_else(|_| Json(HelloState {
            name: "Fallback".into(),
            id: "0000".into(),
            message: "Default message".into(),
        }))
}
