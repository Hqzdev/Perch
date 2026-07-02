use perch_types::api::{RetrievalAnswerRequest, RetrievalAnswerResponse};
use thiserror::Error;
use url::Url;

use crate::domain::messages::{AssistantAnswer, Citation, VisitorMessage};
use crate::domain::sites::Site;

#[derive(Debug, Clone)]
pub struct RetrievalClient {
    client: reqwest::Client,
    base_url: Url,
}

#[derive(Debug, Error)]
pub enum RetrievalClientError {
    #[error("retrieval request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("retrieval url failed: {0}")]
    Url(#[from] url::ParseError),
}

impl RetrievalClient {
    pub fn new(base_url: Url) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
        }
    }

    pub async fn answer(
        &self,
        site: &Site,
        message: &VisitorMessage,
    ) -> Result<AssistantAnswer, RetrievalClientError> {
        let url = self.base_url.join("/v1/answer")?;
        let response = self
            .client
            .post(url)
            .json(&RetrievalAnswerRequest {
                site_id: site.id,
                site_name: site.name.clone(),
                site_origin: site.origin.clone(),
                question: message.content.clone(),
            })
            .send()
            .await?
            .error_for_status()?
            .json::<RetrievalAnswerResponse>()
            .await?;

        Ok(AssistantAnswer {
            conversation_id: uuid::Uuid::nil(),
            message_id: uuid::Uuid::nil(),
            content: response.answer,
            citations: response
                .citations
                .into_iter()
                .map(|citation| Citation {
                    title: citation.title,
                    url: citation.url,
                })
                .collect(),
        })
    }
}
