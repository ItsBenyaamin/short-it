extern crate core;

mod data;
mod short_it;
mod config;
mod api;

use config::AppConfig;
use data::MysqlDB;
use crate::api::short_api::short_api::setup_endpoints;
use crate::short_it::short_it::short_it::ShortItClient;
use crate::short_it::ShortIt;

#[tokio::main]
async fn main() {
    let mut config = AppConfig::create();
    let db_client = MysqlDB::new();
    let short_it = ShortIt::from(db_client, config);
    setup_endpoints(short_it.to_client()).await;
}
