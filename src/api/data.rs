use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AddRequest {
    pub url: String,
    pub until: String,
    pub token: String
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EditRequest {
    pub hash: String,
    pub url: String,
    pub until: String,
    pub token: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RemoveRequest {
    pub hash: String,
    pub token: String
}