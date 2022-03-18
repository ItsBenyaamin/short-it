pub mod short_app {
    use std::sync::Arc;
    use std::time::{SystemTime, UNIX_EPOCH};
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

        pub fn get_url(&self, hash: String, ip: String, referer: String) -> Option<String> {
            if self.db_client.is_hash_exist(&hash){
                if let Some(short) = self.db_client.get_short(&hash) {
                    let now_as_millis = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
                    if !short.until.eq("0") {
                        let short_until = short.until.parse::<u128>().unwrap();
                        if short_until > now_as_millis {
                            // add a until reaction decision in app setting
                            // for now just not redirect to the url and return 404.
                            return None;
                        }
                    }
                    self.db_client.new_analytics(&hash, &ip, &referer, &now_as_millis.to_string());
                    return Some(short.url);
                }
            }
            None
        }

        pub fn login(&mut self) -> String {
            let token = nanoid!(32);
            self.config.renew_token(&token);
            let login_response = LoginData::from(self.config.username.clone(), token.clone());
            let response = Response::with_data(login_response, 200);
            serde_json::to_string(&response).unwrap()
        }

        pub fn list_of_shorts(&mut self) -> String {
            match self.db_client.list_of_all() {
                Some(result) => {
                    serde_json::to_string(&result).unwrap()
                }
                None => {
                    let response = Response::with_error("server failed to provide data!".to_string(), 500, "".to_string());
                    serde_json::to_string(&response).unwrap()
                }
            }
        }

        pub fn short_with(&mut self, url: String, until: String) -> String {
            let hash = nanoid!(6);
            let short = Short {
                hash,
                url: url.clone(),
                until: until.clone(),
                views: 0
            };
            let result = self.db_client.add(short);

            let response = match result {
                ApiOperationStatus::Inserted => {
                    Response::with_data(String::from("inserted.")
                                        , 201)
                }

                ApiOperationStatus::DuplicatedHashError => {
                    return self.short_with(url, until);
                }

                ApiOperationStatus::InsertError |
                    ApiOperationStatus::ConnectionError | _ => {
                    Response::with_error(String::from("please try again later :(."),
                                         500,
                                         "".to_string())
                }

            };
            serde_json::to_string(&response).unwrap()
        }

        pub fn edit_short(&mut self, hash: String, url: String, until: String) -> String {
            let result = self.db_client.edit(hash, url, until);
            let response = match result {
                ApiOperationStatus::Edited => {
                    Response::with_data(String::from("edited.")
                                        , 200)
                }

                ApiOperationStatus::EditError |
                    ApiOperationStatus::ConnectionError | _ => {
                    Response::with_error(String::from("please try again later :(."),
                                         500,
                                         "".to_string())
                }
            };
            serde_json::to_string(&response).unwrap()
        }

        pub fn delete_short(&mut self, hash: String) -> String {
            let result = self.db_client.delete(hash);
            let response = match result {
                ApiOperationStatus::Deleted => {
                    Response::with_data(String::from("deleted.")
                                        , 200)
                }

                ApiOperationStatus::DeleteError |
                    ApiOperationStatus::ConnectionError | _ => {
                    Response::with_error(String::from("please try again later :(."),
                                         500,
                                         "".to_string())
                }
            };
            serde_json::to_string(&response).unwrap()
        }

    }

}