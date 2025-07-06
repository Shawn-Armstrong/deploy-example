use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct HelloState {
    pub name: String,
    pub id: String,
    pub message: String,
}
