pub trait DatabaseInterface {
    fn list_of_all(&self);

    fn add(&self, url: String, hash: String, until: u64) -> String;

    fn edit(&self, id: u64) -> String;

    fn delete(&self, id: u64);
}