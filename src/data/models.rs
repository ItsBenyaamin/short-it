use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Short {
    pub hash: String,
    pub url: String,
    pub until: String,
    pub views: usize
}