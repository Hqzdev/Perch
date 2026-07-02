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
    pub services: UpstreamServiceSettings,
    pub vector_search: VectorSearchSettings,
    pub llm: LlmSettings,
    pub owner_access: OwnerAccessSettings,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataStoreSettings {
    pub database_url: Url,
    pub redis_url: Url,
    pub qdrant_url: Url,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpstreamServiceSettings {
    pub indexer_url: Url,
    pub retrieval_url: Url,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorSearchSettings {
    pub enabled: bool,
    pub url: Url,
    pub collection: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmSettings {
    pub provider: String,
    pub model: String,
    pub api_key: Option<String>,
    pub base_url: Url,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerAccessSettings {
    pub token: String,
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
            services: UpstreamServiceSettings::from_env()?,
            vector_search: VectorSearchSettings::from_env()?,
            llm: LlmSettings::from_env()?,
            owner_access: OwnerAccessSettings::from_env(),
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

impl UpstreamServiceSettings {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            indexer_url: parse_url("PERCH_INDEXER_URL", "http://127.0.0.1:8081")?,
            retrieval_url: parse_url("PERCH_RETRIEVAL_URL", "http://127.0.0.1:8082")?,
        })
    }
}

impl VectorSearchSettings {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            enabled: env_bool("PERCH_QDRANT_ENABLED", true),
            url: parse_url("PERCH_QDRANT_URL", "http://127.0.0.1:6335")?,
            collection: env::var("PERCH_QDRANT_COLLECTION")
                .unwrap_or_else(|_| "perch_chunks".to_string()),
        })
    }
}

impl LlmSettings {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            provider: env::var("PERCH_LLM_PROVIDER")
                .unwrap_or_else(|_| "disabled".to_string())
                .to_lowercase(),
            model: env::var("PERCH_LLM_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string()),
            api_key: env::var("PERCH_LLM_API_KEY")
                .ok()
                .filter(|value| !value.trim().is_empty()),
            base_url: parse_url("PERCH_LLM_BASE_URL", "https://api.openai.com/v1")?,
        })
    }

    pub fn enabled(&self) -> bool {
        self.provider != "disabled" && self.api_key.is_some()
    }
}

impl OwnerAccessSettings {
    pub fn from_env() -> Self {
        Self {
            token: env::var("PERCH_OWNER_TOKEN")
                .unwrap_or_else(|_| "perch_dev_owner_token".to_string()),
        }
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

fn env_bool(variable: &str, fallback: bool) -> bool {
    env::var(variable)
        .ok()
        .and_then(|value| match value.to_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Some(true),
            "0" | "false" | "no" | "off" => Some(false),
            _ => None,
        })
        .unwrap_or(fallback)
}
