use std::sync::Arc;

use thiserror::Error;

use crate::domain::sites::{NewSite, Site};
use crate::infrastructure::storage::{SiteRepository, SiteRepositoryError};

#[derive(Debug, Clone)]
pub struct SiteService {
    repository: Arc<SiteRepository>,
}

#[derive(Debug, Error)]
pub enum SiteServiceError {
    #[error("site payload is invalid")]
    InvalidSite,
    #[error("widget origin is missing")]
    MissingOrigin,
    #[error("widget origin is not allowed")]
    OriginNotAllowed,
    #[error("site was not found")]
    NotFound,
    #[error("site storage failed: {0}")]
    Storage(#[from] SiteRepositoryError),
}

impl SiteService {
    pub fn new(repository: SiteRepository) -> Self {
        Self {
            repository: Arc::new(repository),
        }
    }

    pub async fn create_site(&self, new_site: NewSite) -> Result<Site, SiteServiceError> {
        if !new_site.valid() {
            return Err(SiteServiceError::InvalidSite);
        }

        self.repository
            .create_site(new_site)
            .await
            .map_err(Into::into)
    }

    pub async fn resolve_widget_site(
        &self,
        script_key: &str,
        origin: Option<&str>,
    ) -> Result<Site, SiteServiceError> {
        let origin = origin
            .map(|value| value.trim().trim_end_matches('/'))
            .filter(|value| !value.is_empty())
            .ok_or(SiteServiceError::MissingOrigin)?;
        let site = self
            .repository
            .find_by_script_key(script_key)
            .await?
            .ok_or(SiteServiceError::NotFound)?;

        if site.origin != origin {
            return Err(SiteServiceError::OriginNotAllowed);
        }

        Ok(site)
    }
}
