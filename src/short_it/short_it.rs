pub mod short_it {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use nanoid::nanoid;
    use crate::api::*;
    use crate::{AppConfig, MysqlDB};
    use crate::data::{DatabaseInterface, Short};

    pub type ShortItClient = Arc<Mutex<ShortIt>>;

    #[derive(Debug, Clone)]
    pub struct ShortIt {
        db_client: MysqlDB,
        pub config: AppConfig
    }

    impl ShortIt{

        pub fn from(db_client: MysqlDB, config: AppConfig) -> Self {
            ShortIt {
                db_client,
                config
            }
        }

        pub fn to_client(&self) -> ShortItClient {
            Arc::new(Mutex::new(self.clone()))
        }

        pub fn login(&mut self) -> String {
            let token = nanoid!(32);
            self.config.renew_token(&token);
            let mut login_response = LoginData::from(self.config.username.clone(), token.clone());
            let response = Response::with_data(login_response, 200);
            return serde_json::to_string(&response).unwrap();
        }

        pub fn list_of_shorts(&mut self) -> String {
            return match self.db_client.list_of_all() {
                Some(result) => {
                    serde_json::to_string(&result).unwrap()
                }
                None => {
                    let response = Response::with_error("server failed to provide data!".to_string(), 500, "".to_string());
                    serde_json::to_string(&response).unwrap()
                }
            };
        }

        pub fn short_with(&mut self, url: String, until: u64) -> String {
            let uuid = nanoid!(6);
            self.db_client.add(url, uuid, until, "".to_string())
        }

    }

}