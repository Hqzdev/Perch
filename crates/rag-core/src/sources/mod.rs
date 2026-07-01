use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NormalizedUrl(String);

#[derive(Debug, Error)]
pub enum SourceError {
    #[error("invalid url: {0}")]
    InvalidUrl(String),
    #[error("unsupported url scheme: {0}")]
    UnsupportedScheme(String),
}

impl NormalizedUrl {
    pub fn parse(value: &str) -> Result<Self, SourceError> {
        let parsed = Url::parse(value).map_err(|_| SourceError::InvalidUrl(value.to_owned()))?;
        match parsed.scheme() {
            "http" | "https" => Ok(Self(parsed.to_string())),
            scheme => Err(SourceError::UnsupportedScheme(scheme.to_owned())),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
