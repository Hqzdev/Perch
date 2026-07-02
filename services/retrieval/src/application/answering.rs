use std::sync::Arc;

use perch_types::api::{RetrievalAnswerRequest, RetrievalAnswerResponse, WidgetCitation};
use thiserror::Error;

use crate::domain::context::RetrievedContext;
use crate::infrastructure::storage::{ContextRepository, ContextRepositoryError};

#[derive(Debug, Clone)]
pub struct AnswerService {
    repository: Arc<ContextRepository>,
}

#[derive(Debug, Error)]
pub enum AnswerServiceError {
    #[error("context repository failed: {0}")]
    Context(#[from] ContextRepositoryError),
}

impl AnswerService {
    pub fn new(repository: ContextRepository) -> Self {
        Self {
            repository: Arc::new(repository),
        }
    }

    pub async fn answer(
        &self,
        request: RetrievalAnswerRequest,
    ) -> Result<RetrievalAnswerResponse, AnswerServiceError> {
        let context = self
            .repository
            .search(request.site_id, &request.question)
            .await?;

        Ok(if context.has_sources() {
            sourced_answer(&request, context)
        } else {
            fallback_answer(&request)
        })
    }
}

fn sourced_answer(
    request: &RetrievalAnswerRequest,
    context: RetrievedContext,
) -> RetrievalAnswerResponse {
    let citations = context
        .chunks
        .iter()
        .map(|chunk| WidgetCitation {
            title: chunk.source_title.clone(),
            url: chunk.source_url.clone(),
        })
        .collect::<Vec<_>>();
    let evidence = context
        .chunks
        .iter()
        .map(|chunk| chunk.content.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let summary = evidence.chars().take(420).collect::<String>();

    RetrievalAnswerResponse {
        answer: format!(
            "Based on indexed pages for {}, the closest matching source says: {}",
            request.site_name, summary
        ),
        citations,
    }
}

fn fallback_answer(request: &RetrievalAnswerRequest) -> RetrievalAnswerResponse {
    RetrievalAnswerResponse {
        answer: format!(
            "I could not find indexed page chunks for {} that match this question yet. Index the site content first, then ask again. Question: {}",
            request.site_name,
            request.question.trim()
        ),
        citations: vec![WidgetCitation {
            title: request.site_name.clone(),
            url: request.site_origin.clone(),
        }],
    }
}
