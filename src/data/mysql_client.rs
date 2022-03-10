pub mod mysql_impl {
    use crate::data::{DatabaseInterface, Short};
    use mysql::*;
    use mysql::prelude::*;

    #[derive(Debug, Clone)]
    pub struct MysqlDB {
        pub connection: Pool
    }

    impl MysqlDB {
        pub fn new(username: &String, password: &String, database: &String) -> Self {
            let url = format!("mysql://{}:{}@192.168.1.100:3306/{}", username, password, database);
            let opts = mysql::Opts::from_url(url.as_str()).unwrap();
            let mut pool = match Pool::new(opts) {
                Ok(p) => {
                    p
                }
                Err(e) => {
                    panic!("Error: {}", e);
                }
            };
            MysqlDB {
                connection: pool
            }
        }

    }

    impl DatabaseInterface for MysqlDB {

        fn list_of_all(&mut self) -> Option<Vec<Short>> {
            let mut connection = self.connection.get_conn();
            if connection.is_err() {
                return None;
            }
            let result = connection.unwrap().query_map("select * from shorts", |(hash, url, until, view)| {
                Short {
                    hash,
                    url,
                    until,
                    view
                }
            });

            match result {
                Ok(data) => {
                    Some(data)
                }
                Err(_) => { None }
            }
        }

        fn add(&mut self, url: String, hash: String, until: u64, token: String) -> String {
            todo!()
        }

        fn edit(&mut self, id: u64) -> String {
            todo!()
        }

        fn delete(&mut self, id: u64) {
            todo!()
        }
    }

}