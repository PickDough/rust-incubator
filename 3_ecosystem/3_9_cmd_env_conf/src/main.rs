use std::path::{Path, PathBuf};

use clap::error::ErrorKind;
use clap::Parser;
use clap_derive::Parser;
use config::Config;
use serde::Deserialize;
use smart_default::SmartDefault;

#[derive(Debug, Deserialize, SmartDefault)]
pub struct Settings {
    pub mode: Mode,
    pub server: Server,
    pub db: Db,
    pub log: Log,
    pub background: Option<Background>,
}

#[derive(Debug, Deserialize, SmartDefault)]
pub struct Mode {
    pub debug: bool,
}

#[derive(Debug, Deserialize, SmartDefault)]
pub struct Server {
    #[default = "http://127.0.0.1"]
    pub external_url: String,
    #[default = 8081]
    pub http_port: u16,
    #[default = 8082]
    pub grpc_port: u16,
    #[default = 10025]
    pub healthz_port: u16,
    #[default = 9199]
    pub metrics_port: u16,
}

#[derive(Debug, Deserialize, SmartDefault)]
pub struct Db {
    pub mysql: Mysql,
}

#[derive(Debug, Deserialize, SmartDefault)]
pub struct Mysql {
    #[default = "127.0.0.1"]
    pub host: String,
    #[default = 3306]
    pub port: u16,
    #[default = "default"]
    pub database: String,
    #[default = "root"]
    pub user: String,
    #[default = ""]
    pub pass: String,
    pub connections: Connections,
}

#[derive(Debug, Deserialize, SmartDefault)]
pub struct Connections {
    #[default = 30]
    pub max_idle: u16,
    #[default = 30]
    pub max_open: u16,
}

#[derive(Debug, Deserialize, SmartDefault)]
pub struct Log {
    pub app: App,
}

#[derive(Debug, Deserialize, SmartDefault)]
pub struct App {
    #[default = "info"]
    pub level: String,
}

#[derive(Debug, Deserialize, SmartDefault)]
pub struct Background {
    pub watchdog: Watchdog,
}

#[derive(Debug, Deserialize, SmartDefault)]
pub struct Watchdog {
    #[default = "5s"]
    pub period: String,
    #[default = 10]
    pub limit: u16,
    #[default = "4s"]
    pub lock_timeout: String,
}

#[derive(Debug, Parser)]
#[command(version)]
/// Program to retrieve and output its configuration
pub struct Cli {
    /// Path to the configuration file
    #[clap(short, long, default_value="3_ecosystem/3_9_cmd_env_conf/config.toml", value_parser = validator_path_buf)]
    conf: PathBuf,
}

pub fn validator_path_buf(s: &str) -> Result<PathBuf, clap::Error> {
    let path = Path::new(s).to_path_buf();

    if path.exists() {
        Ok(path)
    } else {
        let err = clap::Error::new(ErrorKind::ValueValidation);
        Err(err)
    }
}

fn main() {
    let cli = Cli::parse();

    let settings = Config::builder()
        .add_source(config::File::from(cli.conf))
        .add_source(config::Environment::with_prefix("CONF"))
        .build()
        .unwrap();

    let settings: Settings = settings.try_deserialize().unwrap();
    
    println!("{:#?}", settings);
}
