use perch_config::VectorSearchSettings;
use rag_core::embeddings::{hash_embedding, HASH_EMBEDDING_DIMENSIONS};
use reqwest::{Client, StatusCode};
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::pages::IndexedChunk;

#[derive(Debug, Clone)]
pub struct QdrantVectorStore {
    client: Client,
    settings: VectorSearchSettings,
}

#[derive(Debug, Error)]
pub enum QdrantVectorStoreError {
    #[error("qdrant request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("qdrant returned {status}: {body}")]
    Response { status: StatusCode, body: String },
}

#[derive(Debug, Serialize)]
struct EnsureCollectionRequest {
    vectors: VectorParams,
}

#[derive(Debug, Serialize)]
struct VectorParams {
    size: usize,
    distance: &'static str,
}

#[derive(Debug, Serialize)]
struct DeletePointsRequest {
    filter: Filter,
}

#[derive(Debug, Serialize)]
struct UpsertPointsRequest {
    points: Vec<Point>,
}

#[derive(Debug, Serialize)]
struct Point {
    id: Uuid,
    vector: Vec<f32>,
    payload: PointPayload,
}

#[derive(Debug, Serialize)]
struct PointPayload {
    site_id: String,
    page_id: String,
    chunk_id: String,
    chunk_index: usize,
    source_url: String,
    source_title: String,
    content: String,
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

impl QdrantVectorStore {
    pub fn new(settings: VectorSearchSettings) -> Self {
        Self {
            client: Client::new(),
            settings,
        }
    }

    pub async fn ready(&self) -> Result<(), QdrantVectorStoreError> {
        if !self.settings.enabled {
            return Ok(());
        }

        let response = self.client.get(self.ready_url()).send().await?;

        self.accept(response).await
    }

    pub async fn upsert_page_chunks(
        &self,
        site_id: Uuid,
        page_id: Uuid,
        chunks: &[IndexedChunk],
    ) -> Result<(), QdrantVectorStoreError> {
        if !self.settings.enabled || chunks.is_empty() {
            return Ok(());
        }

        self.ensure_collection().await?;
        self.delete_page(page_id).await?;
        self.upsert_chunks(site_id, chunks).await
    }

    async fn ensure_collection(&self) -> Result<(), QdrantVectorStoreError> {
        let request = EnsureCollectionRequest {
            vectors: VectorParams {
                size: HASH_EMBEDDING_DIMENSIONS,
                distance: "Cosine",
            },
        };
        let url = self.collection_url();
        let response = self.client.put(url).json(&request).send().await?;

        if response.status().is_success() {
            return Ok(());
        }

        let status = response.status();
        let body = response.text().await.unwrap_or_default();

        if status == StatusCode::CONFLICT || body.contains("already exists") {
            return Ok(());
        }

        Err(QdrantVectorStoreError::Response { status, body })
    }

    async fn delete_page(&self, page_id: Uuid) -> Result<(), QdrantVectorStoreError> {
        let request = DeletePointsRequest {
            filter: Filter {
                must: vec![FieldCondition {
                    key: "page_id",
                    r#match: MatchValue {
                        value: page_id.to_string(),
                    },
                }],
            },
        };
        let url = format!("{}/points/delete?wait=true", self.collection_url());
        let response = self.client.post(url).json(&request).send().await?;

        self.accept(response).await
    }

    async fn upsert_chunks(
        &self,
        site_id: Uuid,
        chunks: &[IndexedChunk],
    ) -> Result<(), QdrantVectorStoreError> {
        let points = chunks
            .iter()
            .map(|chunk| Point {
                id: chunk.chunk_id,
                vector: hash_embedding(&chunk.content).values,
                payload: PointPayload {
                    site_id: site_id.to_string(),
                    page_id: chunk.page_id.to_string(),
                    chunk_id: chunk.chunk_id.to_string(),
                    chunk_index: chunk.chunk_index,
                    source_url: chunk.source_url.clone(),
                    source_title: chunk.source_title.clone(),
                    content: chunk.content.clone(),
                },
            })
            .collect();
        let request = UpsertPointsRequest { points };
        let url = format!("{}/points?wait=true", self.collection_url());
        let response = self.client.put(url).json(&request).send().await?;

        self.accept(response).await
    }

    async fn accept(&self, response: reqwest::Response) -> Result<(), QdrantVectorStoreError> {
        if response.status().is_success() {
            return Ok(());
        }

        let status = response.status();
        let body = response.text().await.unwrap_or_default();

        Err(QdrantVectorStoreError::Response { status, body })
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
