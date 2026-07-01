use std::sync::Arc;

use thiserror::Error;

use crate::domain::messages::{AssistantAnswer, Citation, VisitorMessage};
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
    #[error("message payload is invalid")]
    InvalidMessage,
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

    pub async fn answer_widget_message(
        &self,
        script_key: &str,
        origin: Option<&str>,
        message: VisitorMessage,
    ) -> Result<AssistantAnswer, SiteServiceError> {
        if !message.valid() {
            return Err(SiteServiceError::InvalidMessage);
        }

        let site = self.resolve_widget_site(script_key, origin).await?;
        let draft = Self::draft_answer(&site, &message);

        self.repository
            .record_widget_exchange(site.id, message, draft)
            .await
            .map_err(Into::into)
    }

    fn draft_answer(site: &Site, message: &VisitorMessage) -> AssistantAnswer {
        let content = format!(
            "Perch received your question for {} and stored it for retrieval. The next backend stage will route this message through indexed page chunks and return a cited answer. Your question was: {}",
            site.name, message.content
        );

        AssistantAnswer {
            conversation_id: uuid::Uuid::nil(),
            message_id: uuid::Uuid::nil(),
            content,
            citations: vec![Citation {
                title: site.name.clone(),
                url: site.origin.clone(),
            }],
        }
    }
}
