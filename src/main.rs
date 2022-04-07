mod data;
mod app;
mod app_config;
mod api;
mod utils;

use crate::app_config::AppConfig;
use crate::data::MysqlDB;
use crate::api::short_api::api::setup_endpoints;
use crate::app::short_it::short_app::ShortItClient;
use crate::app::ShortIt;
use crate::utils::encryption_util;
use std::io;
use std::io::Write;
use clap::{arg, Command};
use ipaddress::IPAddress;


const HEADER: &str = r" ________  ___  ___  ________  ________  _________        ___  _________
|\   ____\|\  \|\  \|\   __  \|\   __  \|\___   ___\     |\  \|\___   ___\
\ \  \___|\ \  \\\  \ \  \|\  \ \  \|\  \|___ \  \_|     \ \  \|___ \  \_|
 \ \_____  \ \   __  \ \  \\\  \ \   _  _\   \ \  \       \ \  \   \ \  \
  \|____|\  \ \  \ \  \ \  \\\  \ \  \\  \|   \ \  \       \ \  \   \ \  \
    ____\_\  \ \__\ \__\ \_______\ \__\\ _\    \ \__\       \ \__\   \ \__\
   |\_________\|__|\|__|\|_______|\|__|\|__|    \|__|        \|__|    \|__|
   \|_________|

                                                                           ";

#[tokio::main]
async fn main() {
    let mut config = AppConfig::create();

    let matches = Command::new("short-it")
        .arg(arg!(--db_name <Database_Name>).required(false))
        .arg(arg!(--db_user <Database_Username>).required(false))
        .arg(arg!(--db_pass <Database_Password>).required(false))
        .arg(arg!(--user <Panel_Username>).required(false))
        .arg(arg!(--pass <Panel_Password>).required(false))
        .version("1.0")
        .get_matches();

    if let Some(db_name) = matches.value_of("db_name") {
        println!("Database name updated: {}", db_name);
        config.db_name = db_name.to_owned();
    }

    if let Some(db_user) = matches.value_of("db_user") {
        println!("Database username updated: {}", db_user);
        config.db_username = db_user.to_owned();
    }

    if let Some(db_pass) = matches.value_of("db_pass") {
        println!("Database password updated");
        let encrypted_password = encryption_util::encrypt(db_pass);
        config.db_password = encrypted_password;
    }

    if let Some(user) = matches.value_of("user") {
        println!("Panel username updated: {}", user);
        config.username = user.to_owned();
    }

    if let Some(pass) = matches.value_of("pass") {
        println!("Panel password updated");
        let encrypted_password = encryption_util::encrypt(pass);
        config.password = encrypted_password;
    }

    config.update();

    if !matches.args_present() {
        println!("empty!");
        start(config).await;
    }
}

async fn start(config: AppConfig) {
    println!("{}", HEADER);
    let db_client = MysqlDB::new(&config.db_username, &config.db_password, &config.db_name);
    let short_it = ShortIt::from(db_client, config);
    setup_endpoints(short_it.to_client()).await;
}