use std::fmt::Error;
use crate::api::ApiOperationStatus;
use crate::data::Short;

pub trait DatabaseInterface {
    fn list_of_all(&mut self) -> Option<Vec<Short>>;

    fn is_hash_exist(&self, hash: &String) -> bool;

    fn add(&mut self, short: Short) -> ApiOperationStatus;

    fn edit(&mut self, hash: String, url: String, until: f64) -> ApiOperationStatus;

    fn delete(&mut self, hash: String) -> ApiOperationStatus;
}