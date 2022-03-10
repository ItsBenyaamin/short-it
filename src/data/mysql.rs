pub mod mysql_impl {
    use crate::data::DatabaseInterface;

    #[derive(Debug, Clone)]
    pub struct MysqlDB {

    }

    impl MysqlDB {
        pub fn new() -> Self {
            MysqlDB {}
        }
    }

    impl DatabaseInterface for MysqlDB {

        fn list_of_all(&self) {
            todo!()
        }

        fn add(&self, url: String, hash: String, until: u64) -> String {
            todo!()
        }

        fn edit(&self, id: u64) -> String {
            todo!()
        }

        fn delete(&self, id: u64) {
            todo!()
        }
    }

}