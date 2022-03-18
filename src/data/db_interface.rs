use crate::api::ApiOperationStatus;
use crate::data::Short;

pub trait DatabaseInterface {
    fn list_of_all(&mut self) -> Option<Vec<Short>>;

    fn is_hash_exist(&self, hash: &str) -> bool;

    fn get_short(&self, hash: &str) -> Option<Short>;

    fn new_analytics(&self, hash: &str, ip: &str, referer: &str, time: &str);

    fn add(&mut self, short: Short) -> ApiOperationStatus;

    fn edit(&mut self, hash: String, url: String, until: String) -> ApiOperationStatus;

    fn delete(&mut self, hash: String) -> ApiOperationStatus;
}