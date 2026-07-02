use std::sync::Arc;

use perch_types::api::{CrawlJobResponse, CrawlSiteRequest};
use rag_core::chunking::{chunk_text, ChunkingConfig};
use thiserror::Error;
use uuid::Uuid;

use crate::domain::pages::{text_from_html, IndexedPage, PageDocument};
use crate::infrastructure::crawler::{WebCrawler, WebCrawlerError};
use crate::infrastructure::storage::{PageRepository, PageRepositoryError};

#[derive(Debug, Clone)]
pub struct IndexingService {
    crawler: Arc<WebCrawler>,
    repository: Arc<PageRepository>,
}

#[derive(Debug, Error)]
pub enum IndexingServiceError {
    #[error("page document is invalid")]
    InvalidPage,
    #[error("crawl target is invalid")]
    InvalidCrawlTarget,
    #[error("page fetch failed: {0}")]
    Crawl(#[from] WebCrawlerError),
    #[error("page storage failed: {0}")]
    Storage(#[from] PageRepositoryError),
}

impl IndexingService {
    pub fn new(crawler: WebCrawler, repository: PageRepository) -> Self {
        Self {
            crawler: Arc::new(crawler),
            repository: Arc::new(repository),
        }
    }

    pub async fn index_page(
        &self,
        document: PageDocument,
    ) -> Result<IndexedPage, IndexingServiceError> {
        if !document.valid() {
            return Err(IndexingServiceError::InvalidPage);
        }

        let chunks = chunk_text(&document.content, ChunkingConfig::conservative());
        self.repository
            .upsert_page(document, chunks)
            .await
            .map_err(Into::into)
    }

    pub async fn crawl_page(
        &self,
        site_id: Uuid,
        request: CrawlSiteRequest,
        fallback_url: String,
    ) -> Result<CrawlJobResponse, IndexingServiceError> {
        let url = request.url.unwrap_or(fallback_url);

        if !valid_crawl_url(&url) {
            return Err(IndexingServiceError::InvalidCrawlTarget);
        }

        let job = self.repository.create_crawl_job(site_id, &url).await?;
        self.repository.mark_crawl_running(job.job_id).await?;
        let fetched = match self.crawler.fetch(&url).await {
            Ok(fetched) => fetched,
            Err(error) => {
                return self
                    .repository
                    .mark_crawl_failed(job.job_id, error.to_string())
                    .await
                    .map_err(Into::into);
            }
        };
        let indexed = self
            .index_page(PageDocument::new(
                site_id,
                fetched.url.clone(),
                fetched.title,
                text_from_html(&fetched.html),
            ))
            .await?;

        self.repository
            .mark_crawl_succeeded(job.job_id, indexed.page_id, indexed.chunks_indexed)
            .await
            .map_err(Into::into)
    }

    pub async fn crawl_job(
        &self,
        job_id: Uuid,
    ) -> Result<Option<CrawlJobResponse>, IndexingServiceError> {
        self.repository
            .find_crawl_job(job_id)
            .await
            .map_err(Into::into)
    }
}

fn valid_crawl_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}
