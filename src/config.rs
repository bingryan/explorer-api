use config::{ConfigError, Config, File, Environment};
use std::collections::HashMap;
use std::{env, result};
use std::path::PathBuf;
// use meilisearch_sdk::{document::*, client::*, search::*};

/// Debug only secret for JWT encoding & decoding.
pub const SECRET: &'static str = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=";
pub const CONFIG_FILE: &'static str = "explorer.toml";


/// js toISOString() in test suit can't handle chrono's default precision
pub const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";


pub const TOKEN_PREFIX: &'static str = "Token ";

pub struct AppState {
    pub secret: Vec<u8>,
}


#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub bind_address: String,
    pub secret_key: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExplorerLog {
    pub log_dir: String,
    pub log_cup: usize,
    pub temp_size: String,
    pub zip_compress: bool,
    pub rolling_type: String,
    pub level: String,
    pub debug: bool,
}


#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub log: ExplorerLog,
}

impl Settings {
    pub fn build(file: PathBuf) -> result::Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name(file.into_os_string().into_string().unwrap().as_str()))?;
        s.try_into()
    }
}



