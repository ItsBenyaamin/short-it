pub mod mysql_impl {
    use crate::data::{DatabaseInterface, Short};
    use mysql::*;
    use mysql::prelude::*;
    use crate::api::ApiOperationStatus;

    #[derive(Debug, Clone)]
    pub struct MysqlDB {
        pub connection: Pool
    }

    impl MysqlDB {
        pub fn new(username: &str, password: &str, database: &str) -> Self {
            //TODO get {ip:port} from config file
            let url = format!("mysql://{}:{}@192.168.1.106:3306/{}", username, password, database);
            let opts = mysql::Opts::from_url(url.as_str()).unwrap();
            let pool = match Pool::new(opts) {
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
            let connection = self.connection.get_conn();
            if connection.is_err() {
                return None;
            }
            let mut connection = connection.unwrap();

            let result = connection
                .query_map("select shorts.hash, shorts.url, shorts.until, count(analytics.hash) as views from shorts left join analytics on analytics.hash = shorts.hash group by shorts.hash",
                          |(hash, url, until, views)| {
                              Short {
                                  hash,
                                  url,
                                  until,
                                  views
                              }
                          });

            match result {
                Ok(data) => {
                    Some(data)
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    None
                }
            }
        }

        fn is_hash_exist(&self, hash: &str) -> bool {
            let connection = self.connection.get_conn();
            if connection.is_err() {
                return false;
            }
            let mut connection = connection.unwrap();

            let result: Option<bool> = connection
                .query_first(format!("select exists(select hash from shorts where hash='{}')", hash))
                .unwrap();

            result.unwrap()
        }

        fn get_short(&self, hash: &str) -> Option<Short> {
            let connection = self.connection.get_conn();
            if connection.is_err() {
                return None;
            }
            let mut connection = connection.unwrap();

            let row: Result<Option<Row>> = connection.exec_first("select * from shorts where hash=:hash", params! {
                "hash" => hash
            });

            if row.is_err() { return None; }
            let row = row.unwrap();

            if row.is_none() { return None; }
            let mut row = row.unwrap();

            if row.is_empty() { return None; }

            let r_hash: String = row.take("hash").unwrap();
            let r_url: String = row.take("url").unwrap();
            let r_until: String = row.take("until").unwrap();

            Some(Short {
                hash: r_hash,
                url: r_url,
                until: r_until,
                views: 0
            })
        }

        fn new_analytics(&self, hash: &str, ip: &str, referer: &str, time: &str) {
            let connection = self.connection.get_conn();
            if connection.is_err() {
                return;
            }
            let mut connection = connection.unwrap();

            let _ = connection.exec_drop(
                "insert into analytics (hash, ip, referer, time) values (:hash, :ip, :referer, :time)",
                params! {
                    "hash" => hash,
                    "ip" => ip,
                    "referer" => referer,
                    "time" => time
                }
            );
        }

        fn add(&mut self, short: Short) -> ApiOperationStatus {
            let connection = self.connection.get_conn();
            if connection.is_err() {
                return ApiOperationStatus::ConnectionError;
            }
            let mut connection = connection.unwrap();

            if self.is_hash_exist(&short.hash) {
                return ApiOperationStatus::DuplicatedHashError;
            }

            return match connection.exec_drop(
                "insert into shorts (hash, url, until) values (:hash, :url, :until)",
                params! {
                    "hash" => &short.hash,
                    "url" => short.url,
                    "until" => short.until
                }
            ) {
                Ok(_) => { ApiOperationStatus::Inserted }
                Err(_) => { ApiOperationStatus::InsertError }
            }
        }

        fn edit(&mut self, hash: String, url: String, until: String) -> ApiOperationStatus {
            let connection = self.connection.get_conn();
            if connection.is_err() {
                return ApiOperationStatus::ConnectionError;
            }
            let mut connection = connection.unwrap();
            if self.is_hash_exist(&hash) && connection
                .exec_drop("update shorts set url=:url, until=:until where hash=:hash",
                           params! {
                                "url" => url.trim(),
                                "until" => until,
                                "hash" => hash
                           }).is_ok() {
                return ApiOperationStatus::Edited
            }

            ApiOperationStatus::EditError
        }

        fn delete(&mut self, hash: String) -> ApiOperationStatus {
            let connection = self.connection.get_conn();
            if connection.is_err() {
                return ApiOperationStatus::ConnectionError;
            }
            let mut connection = connection.unwrap();

            if self.is_hash_exist(&hash) && connection
                .exec_drop("delete from shorts where hash =:hash",
                           params! {"hash" => hash}).is_ok() {
                return ApiOperationStatus::Deleted;
            }

            ApiOperationStatus::DeleteError
        }

    }

}