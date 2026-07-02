use perch_config::LlmSettings;
use perch_types::api::RetrievalAnswerRequest;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domain::context::RetrievedContext;

#[derive(Debug, Clone)]
pub struct AnswerGenerator {
    client: Client,
    settings: LlmSettings,
}

#[derive(Debug, Error)]
pub enum AnswerGeneratorError {
    #[error("llm provider is disabled")]
    Disabled,
    #[error("llm provider is unsupported: {0}")]
    UnsupportedProvider(String),
    #[error("llm request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("llm returned {status}: {body}")]
    Response { status: StatusCode, body: String },
    #[error("llm response did not include answer text")]
    EmptyAnswer,
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: &'static str,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatChoiceMessage,
}

#[derive(Debug, Deserialize)]
struct ChatChoiceMessage {
    content: String,
}

impl AnswerGenerator {
    pub fn new(settings: LlmSettings) -> Self {
        Self {
            client: Client::new(),
            settings,
        }
    }

    pub async fn generate(
        &self,
        request: &RetrievalAnswerRequest,
        context: &RetrievedContext,
    ) -> Result<String, AnswerGeneratorError> {
        if !self.settings.enabled() {
            return Err(AnswerGeneratorError::Disabled);
        }

        match self.settings.provider.as_str() {
            "openai" => self.generate_openai(request, context).await,
            provider => Err(AnswerGeneratorError::UnsupportedProvider(
                provider.to_string(),
            )),
        }
    }

    async fn generate_openai(
        &self,
        request: &RetrievalAnswerRequest,
        context: &RetrievedContext,
    ) -> Result<String, AnswerGeneratorError> {
        let api_key = self
            .settings
            .api_key
            .as_ref()
            .ok_or(AnswerGeneratorError::Disabled)?;
        let response = self
            .client
            .post(self.chat_completions_url())
            .bearer_auth(api_key)
            .json(&ChatCompletionRequest {
                model: self.settings.model.clone(),
                messages: vec![
                    ChatMessage {
                        role: "system",
                        content: system_prompt(),
                    },
                    ChatMessage {
                        role: "user",
                        content: user_prompt(request, context),
                    },
                ],
                temperature: 0.2,
            })
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();

            return Err(AnswerGeneratorError::Response { status, body });
        }

        response
            .json::<ChatCompletionResponse>()
            .await?
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.message.content.trim().to_string())
            .filter(|answer| !answer.is_empty())
            .ok_or(AnswerGeneratorError::EmptyAnswer)
    }

    fn chat_completions_url(&self) -> String {
        format!(
            "{}/chat/completions",
            self.settings.base_url.as_str().trim_end_matches('/')
        )
    }
}

fn system_prompt() -> String {
    "You answer website visitor questions using only the provided source excerpts. If the sources do not contain the answer, say that the indexed site content does not answer it. Keep the answer concise and do not invent facts.".to_string()
}

fn user_prompt(request: &RetrievalAnswerRequest, context: &RetrievedContext) -> String {
    let sources = context
        .chunks
        .iter()
        .enumerate()
        .map(|(index, chunk)| {
            format!(
                "Source {}: {}\nURL: {}\nExcerpt: {}",
                index + 1,
                chunk.source_title,
                chunk.source_url,
                chunk.content
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        "Site: {}\nQuestion: {}\n\nSources:\n{}",
        request.site_name,
        request.question.trim(),
        sources
    )
}
