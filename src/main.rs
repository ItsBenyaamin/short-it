mod data;
mod app;
mod app_config;
mod api;

use app_config::AppConfig;
use data::MysqlDB;
use crate::api::short_api::api::setup_endpoints;
use crate::app::short_it::short_app::ShortItClient;
use crate::app::ShortIt;

#[tokio::main]
async fn main() {
    let config = AppConfig::create();
    let db_client = MysqlDB::new(&config.db_username, &config.db_password, &config.db_name);
    let short_it = ShortIt::from(db_client, config);
    setup_endpoints(short_it.to_client()).await;
}
