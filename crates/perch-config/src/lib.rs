use std::env;
use std::net::{AddrParseError, SocketAddr};

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceSettings {
    pub name: String,
    pub bind_addr: SocketAddr,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid socket address in {variable}: {source}")]
    InvalidSocketAddress {
        variable: String,
        source: AddrParseError,
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

pub fn env_key_prefix(value: &str) -> String {
    value.replace('-', "_").to_uppercase()
}
