use perch_storage::Database;
use perch_types::api::{CrawlJobResponse, CrawlJobStatus};
use rag_core::chunking::TextChunk;
use sqlx::Row;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::pages::{IndexedChunk, IndexedPage, PageDocument};

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

        let mut indexed_chunks = Vec::with_capacity(chunks.len());

        for chunk in &chunks {
            let chunk_id = Uuid::new_v4();

            sqlx::query(
                "insert into page_chunks (id, page_id, chunk_index, content, token_count, source_url, source_title)
                 values ($1, $2, $3, $4, $5, $6, $7)",
            )
            .bind(chunk_id)
            .bind(page_id)
            .bind(chunk.index as i32)
            .bind(&chunk.text)
            .bind(token_count(&chunk.text))
            .bind(&document.url)
            .bind(&document.title)
            .execute(&mut *transaction)
            .await?;

            indexed_chunks.push(IndexedChunk {
                chunk_id,
                page_id,
                chunk_index: chunk.index,
                content: chunk.text.clone(),
                source_url: document.url.clone(),
                source_title: document
                    .title
                    .clone()
                    .unwrap_or_else(|| document.url.clone()),
            });
        }

        transaction.commit().await?;

        Ok(IndexedPage {
            page_id,
            chunks_indexed: chunks.len(),
            chunks: indexed_chunks,
        })
    }

    pub async fn create_crawl_job(
        &self,
        site_id: Uuid,
        target_url: &str,
    ) -> Result<CrawlJobResponse, PageRepositoryError> {
        let row = sqlx::query(
            "insert into crawl_jobs (id, site_id, target_url, status)
             values ($1, $2, $3, 'pending')
             returning id, site_id, target_url, status, page_id, pages_indexed, chunks_indexed, error_message",
        )
        .bind(Uuid::new_v4())
        .bind(site_id)
        .bind(target_url)
        .fetch_one(self.database.pool())
        .await?;

        Ok(crawl_job_from_row(row))
    }

    pub async fn mark_crawl_running(&self, job_id: Uuid) -> Result<(), PageRepositoryError> {
        sqlx::query(
            "update crawl_jobs set status = 'running', started_at = now(), updated_at = now() where id = $1",
        )
        .bind(job_id)
        .execute(self.database.pool())
        .await?;

        Ok(())
    }

    pub async fn mark_crawl_succeeded(
        &self,
        job_id: Uuid,
        page_id: Uuid,
        chunks_indexed: usize,
    ) -> Result<CrawlJobResponse, PageRepositoryError> {
        let row = sqlx::query(
             "update crawl_jobs
             set status = 'succeeded', page_id = $2, pages_indexed = 1, chunks_indexed = $3, error_message = null, finished_at = now(), updated_at = now()
             where id = $1
             returning id, site_id, target_url, status, page_id, pages_indexed, chunks_indexed, error_message",
        )
        .bind(job_id)
        .bind(page_id)
        .bind(chunks_indexed as i32)
        .fetch_one(self.database.pool())
        .await?;

        Ok(crawl_job_from_row(row))
    }

    pub async fn mark_crawl_failed(
        &self,
        job_id: Uuid,
        error_message: String,
    ) -> Result<CrawlJobResponse, PageRepositoryError> {
        let row = sqlx::query(
             "update crawl_jobs
             set status = 'failed', error_message = $2, finished_at = now(), updated_at = now()
             where id = $1
             returning id, site_id, target_url, status, page_id, pages_indexed, chunks_indexed, error_message",
        )
        .bind(job_id)
        .bind(error_message)
        .fetch_one(self.database.pool())
        .await?;

        Ok(crawl_job_from_row(row))
    }

    pub async fn find_crawl_job(
        &self,
        job_id: Uuid,
    ) -> Result<Option<CrawlJobResponse>, PageRepositoryError> {
        let row = sqlx::query(
            "select id, site_id, target_url, status, page_id, pages_indexed, chunks_indexed, error_message
             from crawl_jobs
             where id = $1",
        )
        .bind(job_id)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(row.map(crawl_job_from_row))
    }
}

fn crawl_job_from_row(row: sqlx::postgres::PgRow) -> CrawlJobResponse {
    CrawlJobResponse {
        job_id: row.get("id"),
        site_id: row.get("site_id"),
        url: row.get("target_url"),
        status: crawl_job_status(row.get::<String, _>("status").as_str()),
        page_id: row.get("page_id"),
        pages_indexed: row.get::<i32, _>("pages_indexed") as usize,
        chunks_indexed: row.get::<i32, _>("chunks_indexed") as usize,
        error_message: row.get("error_message"),
    }
}

fn crawl_job_status(value: &str) -> CrawlJobStatus {
    match value {
        "pending" => CrawlJobStatus::Pending,
        "running" => CrawlJobStatus::Running,
        "succeeded" => CrawlJobStatus::Succeeded,
        "failed" => CrawlJobStatus::Failed,
        _ => CrawlJobStatus::Failed,
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
