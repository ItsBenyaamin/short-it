use crate::data::Short;

pub trait DatabaseInterface {
    fn list_of_all(&mut self) -> Option<Vec<Short>>;

    fn add(&mut self, url: String, hash: String, until: u64, token: String) -> String;

    fn edit(&mut self, id: u64) -> String;

    fn delete(&mut self, id: u64);
}