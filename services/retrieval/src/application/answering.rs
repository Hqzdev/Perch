use std::sync::Arc;

use perch_types::api::{RetrievalAnswerRequest, RetrievalAnswerResponse, WidgetCitation};
use thiserror::Error;

use crate::domain::context::RetrievedContext;
use crate::infrastructure::llm::{AnswerGenerator, AnswerGeneratorError};
use crate::infrastructure::qdrant::{QdrantContextRepository, QdrantContextRepositoryError};
use crate::infrastructure::storage::{ContextRepository, ContextRepositoryError};

#[derive(Debug, Clone)]
pub struct AnswerService {
    repository: Arc<ContextRepository>,
    vectors: Arc<QdrantContextRepository>,
    generator: Arc<AnswerGenerator>,
}

#[derive(Debug, Error)]
pub enum AnswerServiceError {
    #[error("context repository failed: {0}")]
    Context(#[from] ContextRepositoryError),
    #[error("vector repository failed: {0}")]
    Vector(#[from] QdrantContextRepositoryError),
    #[error("answer generator failed: {0}")]
    Generator(#[from] AnswerGeneratorError),
}

impl AnswerService {
    pub fn new(
        repository: ContextRepository,
        vectors: QdrantContextRepository,
        generator: AnswerGenerator,
    ) -> Self {
        Self {
            repository: Arc::new(repository),
            vectors: Arc::new(vectors),
            generator: Arc::new(generator),
        }
    }

    pub async fn answer(
        &self,
        request: RetrievalAnswerRequest,
    ) -> Result<RetrievalAnswerResponse, AnswerServiceError> {
        let vector_context = match self
            .vectors
            .search(request.site_id, &request.question)
            .await
        {
            Ok(context) => context,
            Err(error) => {
                tracing::warn!(error = %error, site_id = %request.site_id, "vector search failed");
                RetrievedContext::empty()
            }
        };
        let context = if vector_context.has_sources() {
            tracing::info!(
                site_id = %request.site_id,
                chunks = vector_context.chunks.len(),
                retrieval_path = "qdrant",
                "retrieval context selected"
            );
            vector_context
        } else {
            let fallback_context = self
                .repository
                .search(request.site_id, &request.question)
                .await?;

            if fallback_context.has_sources() {
                tracing::info!(
                    site_id = %request.site_id,
                    chunks = fallback_context.chunks.len(),
                    retrieval_path = "postgres",
                    "retrieval context selected"
                );
            } else {
                tracing::info!(
                    site_id = %request.site_id,
                    retrieval_path = "none",
                    "retrieval context missing"
                );
            }

            fallback_context
        };

        Ok(if context.has_sources() {
            match self.generator.generate(&request, &context).await {
                Ok(answer) => {
                    tracing::info!(site_id = %request.site_id, "llm answer generated");
                    generated_answer(answer, context)
                }
                Err(AnswerGeneratorError::Disabled) => {
                    tracing::info!(site_id = %request.site_id, "deterministic answer generated");
                    sourced_answer(&request, context)
                }
                Err(error) => {
                    tracing::warn!(error = %error, site_id = %request.site_id, "llm generation skipped");
                    sourced_answer(&request, context)
                }
            }
        } else {
            fallback_answer(&request)
        })
    }

    pub async fn vector_ready(&self) -> Result<(), QdrantContextRepositoryError> {
        self.vectors.ready().await
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

fn generated_answer(answer: String, context: RetrievedContext) -> RetrievalAnswerResponse {
    RetrievalAnswerResponse {
        answer,
        citations: citations_from_context(&context),
    }
}

fn citations_from_context(context: &RetrievedContext) -> Vec<WidgetCitation> {
    context
        .chunks
        .iter()
        .map(|chunk| WidgetCitation {
            title: chunk.source_title.clone(),
            url: chunk.source_url.clone(),
        })
        .collect()
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
