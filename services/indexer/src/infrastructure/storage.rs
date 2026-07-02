use perch_storage::Database;
use rag_core::chunking::TextChunk;
use sqlx::Row;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::pages::{IndexedPage, PageDocument};

#[derive(Debug, Clone)]
pub struct PageRepository {
    database: Database,
}

#[derive(Debug, Error)]
pub enum PageRepositoryError {
    #[error("database query failed: {0}")]
    Query(#[from] sqlx::Error),
}

impl PageRepository {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub async fn upsert_page(
        &self,
        document: PageDocument,
        chunks: Vec<TextChunk>,
    ) -> Result<IndexedPage, PageRepositoryError> {
        let mut transaction = self.database.pool().begin().await?;
        let page_id = Uuid::new_v4();
        let row = sqlx::query(
            "insert into site_pages (id, site_id, url, title, content_hash, status, last_indexed_at, updated_at)
             values ($1, $2, $3, $4, $5, 'indexed', now(), now())
             on conflict (site_id, url) do update set title = excluded.title, content_hash = excluded.content_hash, status = 'indexed', last_indexed_at = now(), updated_at = now()
             returning id",
        )
        .bind(page_id)
        .bind(document.site_id)
        .bind(&document.url)
        .bind(&document.title)
        .bind(content_hash(&document.content))
        .fetch_one(&mut *transaction)
        .await?;
        let page_id = row.get("id");

        sqlx::query("delete from page_chunks where page_id = $1")
            .bind(page_id)
            .execute(&mut *transaction)
            .await?;

        for chunk in &chunks {
            sqlx::query(
                "insert into page_chunks (id, page_id, chunk_index, content, token_count, source_url, source_title)
                 values ($1, $2, $3, $4, $5, $6, $7)",
            )
            .bind(Uuid::new_v4())
            .bind(page_id)
            .bind(chunk.index as i32)
            .bind(&chunk.text)
            .bind(token_count(&chunk.text))
            .bind(&document.url)
            .bind(&document.title)
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;

        Ok(IndexedPage {
            page_id,
            chunks_indexed: chunks.len(),
        })
    }
}

fn token_count(value: &str) -> i32 {
    value.split_whitespace().count() as i32
}

fn content_hash(value: &str) -> String {
    format!(
        "{:016x}",
        value.bytes().fold(0xcbf29ce484222325u64, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
        })
    )
}
