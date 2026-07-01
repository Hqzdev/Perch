use std::env;
use std::net::{AddrParseError, SocketAddr};

use thiserror::Error;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceSettings {
    pub name: String,
    pub bind_addr: SocketAddr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSettings {
    pub service: ServiceSettings,
    pub environment: String,
    pub data_stores: DataStoreSettings,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataStoreSettings {
    pub database_url: Url,
    pub redis_url: Url,
    pub qdrant_url: Url,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid socket address in {variable}: {source}")]
    InvalidSocketAddress {
        variable: String,
        source: AddrParseError,
    },
    #[error("invalid url in {variable}: {source}")]
    InvalidUrl {
        variable: String,
        source: url::ParseError,
    },
}

impl ServiceSettings {
    pub fn from_env(name: impl Into<String>, default_port: u16) -> Result<Self, ConfigError> {
        let name = name.into();
        let variable = format!("{}_BIND_ADDR", env_key_prefix(&name));
        let fallback = format!("127.0.0.1:{default_port}");
        let value = env::var(&variable).unwrap_or(fallback);
        let bind_addr = value
            .parse()
            .map_err(|source| ConfigError::InvalidSocketAddress { variable, source })?;

        Ok(Self { name, bind_addr })
    }
}

impl RuntimeSettings {
    pub fn from_env(name: impl Into<String>, default_port: u16) -> Result<Self, ConfigError> {
        Ok(Self {
            service: ServiceSettings::from_env(name, default_port)?,
            environment: env::var("PERCH_ENV").unwrap_or_else(|_| "development".to_string()),
            data_stores: DataStoreSettings::from_env()?,
        })
    }
}

impl DataStoreSettings {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            database_url: parse_url(
                "PERCH_DATABASE_URL",
                "postgres://perch:perch@127.0.0.1:5433/perch",
            )?,
            redis_url: parse_url("PERCH_REDIS_URL", "redis://127.0.0.1:6380")?,
            qdrant_url: parse_url("PERCH_QDRANT_URL", "http://127.0.0.1:6335")?,
        })
    }
}

pub fn env_key_prefix(value: &str) -> String {
    value.replace('-', "_").to_uppercase()
}

fn parse_url(variable: &str, fallback: &str) -> Result<Url, ConfigError> {
    let value = env::var(variable).unwrap_or_else(|_| fallback.to_string());
    value.parse().map_err(|source| ConfigError::InvalidUrl {
        variable: variable.to_string(),
        source,
    })
}
