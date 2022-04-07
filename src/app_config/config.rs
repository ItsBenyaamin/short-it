pub mod app_config {
    use std::path::{Path, PathBuf};
    use std::fs::OpenOptions;
    use std::io::{Read, Write};
    use serde::{Serialize, Deserialize};
    use serde_json;
    use crate::encryption_util::encrypt;


    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct AppConfig {
        pub config_path: String,
        pub ip: String,
        pub db_name: String,
        pub db_username: String,
        pub db_password: String,
        pub username: String,
        pub password: String,
        pub token: String,
    }


    impl AppConfig {

        pub fn create() -> Self {
            let config_folder = dirs::config_dir();
            let config_file = match config_folder {
                Some(mut config) => {
                    if !config.exists() {
                        let _ = std::fs::create_dir(&config);
                    }
                    config.push("short_it.env");
                    config
                }
                None => {
                    println!("problem due to open userspace app_config folder");
                    panic!();
                }
            };

            if !config_file.exists() {
                let mut default = AppConfig::default_config();
                default.config_path = config_file.as_path().to_str().unwrap().to_string();
                write_config(&config_file, &default);
                return default;
            }

            get_config_from_file(&config_file)
        }

        pub fn renew_token(&mut self, new_token: &str) {
            self.token = new_token.to_string();
            let file = PathBuf::from(&self.config_path);
            write_config(&file, self)
        }

        pub fn update(&mut self) {
            let file = PathBuf::from(&self.config_path);
            write_config(&file, &self)
        }

        pub fn default_config() -> Self {
            AppConfig {
                config_path: String::from(""),
                ip : String::from(""),
                db_name: String::from(""),
                db_username: String::from(""),
                db_password: String::from(""),
                username: String::from("admin"),
                password: encrypt("admin"),
                token: String::from("")
            }
        }

    }

    fn write_config(path_buf: &Path, config: &AppConfig) {
        let options = OpenOptions::new().create(true).write(true).truncate(true).open(&path_buf);
        if let Ok(mut file) = options {
            file.write_all(b"");
            let config_string = serde_json::to_string(&config).unwrap();
            if file.write_all(config_string.as_bytes()).is_err() {
                println!("problem due to write app_config file!");
            }
            file.flush();
        }
    }

    fn get_config_from_file(path_buf: &Path) -> AppConfig {
        let mut file = OpenOptions::new().read(true).open(&path_buf).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        match serde_json::from_str::<AppConfig>(&buf) {
            Ok(config) => { config }
            Err(_) => { AppConfig::default_config() }
        }
    }

}