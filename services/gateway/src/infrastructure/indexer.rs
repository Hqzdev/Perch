use perch_types::api::{
    CrawlJobRequest, CrawlJobResponse, CrawlSiteRequest, IndexPageRequest, IndexPageResponse,
    IndexSitePageRequest,
};
use thiserror::Error;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct IndexerClient {
    client: reqwest::Client,
    base_url: Url,
}

#[derive(Debug, Error)]
pub enum IndexerClientError {
    #[error("indexer request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("indexer url failed: {0}")]
    Url(#[from] url::ParseError),
}

impl IndexerClient {
    pub fn new(base_url: Url) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
        }
    }

    pub async fn index_page(
        &self,
        site_id: Uuid,
        request: IndexSitePageRequest,
    ) -> Result<IndexPageResponse, IndexerClientError> {
        let url = self.base_url.join("/v1/index/pages")?;
        let response = self
            .client
            .post(url)
            .json(&IndexPageRequest {
                site_id,
                url: request.url,
                title: request.title,
                content: request.content,
                content_type: request.content_type,
            })
            .send()
            .await?
            .error_for_status()?
            .json::<IndexPageResponse>()
            .await?;

        Ok(response)
    }

    pub async fn crawl_page(
        &self,
        site_id: Uuid,
        request: CrawlSiteRequest,
        fallback_url: String,
    ) -> Result<CrawlJobResponse, IndexerClientError> {
        let url = self.base_url.join("/v1/crawl/jobs")?;
        let response = self
            .client
            .post(url)
            .json(&CrawlJobRequest {
                site_id,
                url: request.url.unwrap_or(fallback_url),
            })
            .send()
            .await?
            .error_for_status()?
            .json::<CrawlJobResponse>()
            .await?;

        Ok(response)
    }
}
