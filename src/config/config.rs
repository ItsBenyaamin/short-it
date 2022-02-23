pub mod app_config {
    use std::path::PathBuf;
    use std::fs::OpenOptions;
    use std::io::{Read, Write};
    use serde::{Serialize, Deserialize};
    use serde_json;
    use bcrypt::{hash, DEFAULT_COST};


    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct AppConfig {
        pub config_path: String,
        pub ip: String,
        pub db_username: String,
        pub db_password: String,
        pub username: String,
        pub password: String,
        pub token: String,
        pub caching: bool,
    }


    impl AppConfig {

        pub fn create() -> Self {
            let config_folder = dirs::config_dir();
            let config_file = match config_folder {
                Some(mut config) => {
                    if !config.exists() {
                        std::fs::create_dir(&config);
                    }
                    config.push("short_it.env");
                    config
                }
                None => {
                    println!("problem due to open userspace config folder");
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

        pub fn renew_token(&mut self, new_token: &String) {
            self.token = new_token.clone();
            let file = PathBuf::from(&self.config_path);
            write_config(&file, &self)
        }

        fn default_config() -> Self {
            AppConfig {
                config_path: String::from(""),
                ip : String::from(""),
                db_username: String::from(""),
                db_password: String::from(""),
                username: String::from("admin"),
                password: hash("admin", DEFAULT_COST).unwrap(),
                token: String::from(""),
                caching: false
            }
        }

    }

    fn write_config(path_buf: &PathBuf, config: &AppConfig) {
        let options = OpenOptions::new().create(true).write(true).open(&path_buf);
        match options {
            Ok(mut file) => {
                let config_string = serde_json::to_string(&config).unwrap();
                if file.write_all(&config_string.as_bytes()).is_err() {
                    println!("problem due to write config file!");
                }
            }
            Err(e) => {}
        }
    }

    fn get_config_from_file(path_buf: &PathBuf) -> AppConfig {
        let mut file = OpenOptions::new().read(true).open(&path_buf).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        match serde_json::from_str::<AppConfig>(&buf) {
            Ok(config) => { config }
            Err(_) => { AppConfig::default_config() }
        }
    }

}