use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TestState {
    pub name: String,
    pub id: String,
    pub message: String,
}
