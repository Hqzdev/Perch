use std::sync::Arc;

use rag_core::chunking::{chunk_text, ChunkingConfig};
use thiserror::Error;

use crate::domain::pages::{IndexedPage, PageDocument};
use crate::infrastructure::storage::{PageRepository, PageRepositoryError};

#[derive(Debug, Clone)]
pub struct IndexingService {
    repository: Arc<PageRepository>,
}

#[derive(Debug, Error)]
pub enum IndexingServiceError {
    #[error("page document is invalid")]
    InvalidPage,
    #[error("page storage failed: {0}")]
    Storage(#[from] PageRepositoryError),
}

impl IndexingService {
    pub fn new(repository: PageRepository) -> Self {
        Self {
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
}
