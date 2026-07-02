use perch_config::VectorSearchSettings;
use rag_core::embeddings::hash_embedding;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::domain::context::{RetrievedContext, SourceChunk};

#[derive(Debug, Clone)]
pub struct QdrantContextRepository {
    client: Client,
    settings: VectorSearchSettings,
}

#[derive(Debug, Error)]
pub enum QdrantContextRepositoryError {
    #[error("qdrant request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("qdrant returned {status}: {body}")]
    Response { status: StatusCode, body: String },
}

#[derive(Debug, Serialize)]
struct SearchRequest {
    vector: Vec<f32>,
    limit: usize,
    with_payload: bool,
    filter: Filter,
}

#[derive(Debug, Serialize)]
struct Filter {
    must: Vec<FieldCondition>,
}

#[derive(Debug, Serialize)]
struct FieldCondition {
    key: &'static str,
    r#match: MatchValue,
}

#[derive(Debug, Serialize)]
struct MatchValue {
    value: String,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    result: Vec<ScoredPoint>,
}

#[derive(Debug, Deserialize)]
struct ScoredPoint {
    payload: Option<PointPayload>,
}

#[derive(Debug, Deserialize)]
struct PointPayload {
    content: String,
    source_url: String,
    source_title: String,
}

impl QdrantContextRepository {
    pub fn new(settings: VectorSearchSettings) -> Self {
        Self {
            client: Client::new(),
            settings,
        }
    }

    pub async fn ready(&self) -> Result<(), QdrantContextRepositoryError> {
        if !self.settings.enabled {
            return Ok(());
        }

        let response = self.client.get(self.ready_url()).send().await?;

        self.accept(response).await
    }

    pub async fn search(
        &self,
        site_id: Uuid,
        question: &str,
    ) -> Result<RetrievedContext, QdrantContextRepositoryError> {
        if !self.settings.enabled {
            return Ok(RetrievedContext::empty());
        }

        let request = SearchRequest {
            vector: hash_embedding(question).values,
            limit: 4,
            with_payload: true,
            filter: Filter {
                must: vec![FieldCondition {
                    key: "site_id",
                    r#match: MatchValue {
                        value: site_id.to_string(),
                    },
                }],
            },
        };
        let response = self
            .client
            .post(format!("{}/points/search", self.collection_url()))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();

            return Err(QdrantContextRepositoryError::Response { status, body });
        }

        let response = response.json::<SearchResponse>().await?;
        let chunks = response
            .result
            .into_iter()
            .filter_map(|point| point.payload)
            .map(|payload| SourceChunk {
                content: payload.content,
                source_url: payload.source_url,
                source_title: payload.source_title,
            })
            .collect();

        Ok(RetrievedContext { chunks })
    }

    async fn accept(
        &self,
        response: reqwest::Response,
    ) -> Result<(), QdrantContextRepositoryError> {
        if response.status().is_success() {
            return Ok(());
        }

        let status = response.status();
        let body = response.text().await.unwrap_or_default();

        Err(QdrantContextRepositoryError::Response { status, body })
    }

    fn collection_url(&self) -> String {
        format!(
            "{}/collections/{}",
            self.settings.url.as_str().trim_end_matches('/'),
            self.settings.collection
        )
    }

    fn ready_url(&self) -> String {
        format!(
            "{}/readyz",
            self.settings.url.as_str().trim_end_matches('/')
        )
    }
}
