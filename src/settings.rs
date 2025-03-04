use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Database {
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Server {
    pub address: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Logging {
    pub log_level: Option<String>
}

fn default_logging() -> Logging {
    Logging {
        log_level: Some("INFO".to_string()),
    }
}

fn default_server() -> Server {
    Server {
        address: Some("127.0.0.1".to_string()),
        port: Some(8080),
    }
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Settings {
    #[serde(default)]
    pub database: Database,
    #[serde(default = "default_server")]
    pub server: Server,
    #[serde(default = "default_logging")]
    pub logging: Logging,
}

impl Settings {
    pub fn new(location: &str, env_prefix: &str) -> anyhow::Result<Self> {
        let s = Config::builder()
            .add_source(File::with_name(location))
            .add_source(
                Environment::with_prefix(env_prefix)
                    .separator("__")
                    .prefix_separator("__"),
            )
            .set_override("config.location", location)?
            .set_override("config.env_prefix", env_prefix)?
            .build()?;

        let settings = s.try_deserialize()?;

        Ok(settings)
    }
}