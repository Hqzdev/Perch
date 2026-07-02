use perch_storage::Database;
use sqlx::Row;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::context::{RetrievedContext, SourceChunk};

#[derive(Debug, Clone)]
pub struct ContextRepository {
    database: Database,
}

#[derive(Debug, Error)]
pub enum ContextRepositoryError {
    #[error("context query failed: {0}")]
    Query(#[from] sqlx::Error),
}

impl ContextRepository {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub async fn search(
        &self,
        site_id: Uuid,
        question: &str,
    ) -> Result<RetrievedContext, ContextRepositoryError> {
        let terms = search_terms(question);

        if terms.is_empty() {
            return Ok(RetrievedContext::empty());
        }

        let rows = sqlx::query(
            "select pc.content, pc.source_url, coalesce(pc.source_title, sp.title, pc.source_url) as source_title
             from page_chunks pc
             join site_pages sp on sp.id = pc.page_id
             where sp.site_id = $1
             order by pc.created_at desc
             limit 64",
        )
        .bind(site_id)
        .fetch_all(self.database.pool())
        .await?;

        let mut chunks = rows
            .into_iter()
            .map(|row| SourceChunk {
                content: row.get("content"),
                source_url: row.get("source_url"),
                source_title: row.get("source_title"),
            })
            .filter(|chunk| score_chunk(chunk, &terms) > 0)
            .collect::<Vec<_>>();

        chunks.sort_by_key(|chunk| -score_chunk(chunk, &terms));
        chunks.truncate(4);

        Ok(RetrievedContext { chunks })
    }
}

fn search_terms(question: &str) -> Vec<String> {
    question
        .split(|character: char| !character.is_alphanumeric())
        .map(str::to_lowercase)
        .filter(|term| term.len() > 2)
        .take(8)
        .collect()
}

fn score_chunk(chunk: &SourceChunk, terms: &[String]) -> i32 {
    let haystack = format!(
        "{} {} {}",
        chunk.content.to_lowercase(),
        chunk.source_title.to_lowercase(),
        chunk.source_url.to_lowercase()
    );

    terms
        .iter()
        .map(|term| if haystack.contains(term) { 1 } else { 0 })
        .sum()
}
