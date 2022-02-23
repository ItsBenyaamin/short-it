pub mod mysql_impl {

    #[derive(Debug, Clone)]
    pub struct MysqlDB {

    }

    impl MysqlDB {

        pub fn new() -> Self {
            MysqlDB {}
        }

        pub fn list_of_all(&self) -> String {
            todo!()
        }

        pub fn add(&self, url: String, hash: String, until: u64) -> String {
            todo!()
        }

        pub fn edit(&self, id: u64) -> String {
            todo!()
        }

        pub fn delete(&self, id: u64) {
            todo!()
        }

    }

}