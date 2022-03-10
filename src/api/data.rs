use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AddRequest {
    pub url: String,
    pub until: f64,
    pub token: String
}