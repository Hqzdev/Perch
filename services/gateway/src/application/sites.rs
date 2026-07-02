use std::sync::Arc;

use perch_types::api::{
    CrawlJobResponse, CrawlSiteRequest, IndexPageResponse, IndexSitePageRequest,
};
use thiserror::Error;
use uuid::Uuid;

use crate::domain::messages::{AssistantAnswer, VisitorMessage};
use crate::domain::sites::{NewSite, Site};
use crate::infrastructure::indexer::{IndexerClient, IndexerClientError};
use crate::infrastructure::retrieval::{RetrievalClient, RetrievalClientError};
use crate::infrastructure::storage::{SiteRepository, SiteRepositoryError};

#[derive(Debug, Clone)]
pub struct SiteService {
    repository: Arc<SiteRepository>,
    indexer: Arc<IndexerClient>,
    retrieval: Arc<RetrievalClient>,
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
    #[error("indexer request failed: {0}")]
    Indexer(#[from] IndexerClientError),
    #[error("retrieval request failed: {0}")]
    Retrieval(#[from] RetrievalClientError),
}

impl SiteService {
    pub fn new(
        repository: SiteRepository,
        indexer: IndexerClient,
        retrieval: RetrievalClient,
    ) -> Self {
        Self {
            repository: Arc::new(repository),
            indexer: Arc::new(indexer),
            retrieval: Arc::new(retrieval),
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
        let draft = self.retrieval.answer(&site, &message).await?;

        self.repository
            .record_widget_exchange(site.id, message, draft)
            .await
            .map_err(Into::into)
    }

    pub async fn index_site_page(
        &self,
        site_id: Uuid,
        request: IndexSitePageRequest,
    ) -> Result<IndexPageResponse, SiteServiceError> {
        self.repository
            .find_by_id(site_id)
            .await?
            .ok_or(SiteServiceError::NotFound)?;

        self.indexer
            .index_page(site_id, request)
            .await
            .map_err(Into::into)
    }

    pub async fn crawl_site_page(
        &self,
        site_id: Uuid,
        request: CrawlSiteRequest,
    ) -> Result<CrawlJobResponse, SiteServiceError> {
        let site = self
            .repository
            .find_by_id(site_id)
            .await?
            .ok_or(SiteServiceError::NotFound)?;

        self.indexer
            .crawl_page(site_id, request, site.origin)
            .await
            .map_err(Into::into)
    }

    pub async fn crawl_job(
        &self,
        site_id: Uuid,
        job_id: Uuid,
    ) -> Result<CrawlJobResponse, SiteServiceError> {
        self.repository
            .find_by_id(site_id)
            .await?
            .ok_or(SiteServiceError::NotFound)?;

        let job = self.indexer.crawl_job(job_id).await?;

        if job.site_id != site_id {
            return Err(SiteServiceError::NotFound);
        }

        Ok(job)
    }
}
