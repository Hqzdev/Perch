use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisitorMessage {
    pub session_id: Option<String>,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssistantAnswer {
    pub conversation_id: Uuid,
    pub message_id: Uuid,
    pub content: String,
    pub citations: Vec<Citation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Citation {
    pub title: String,
    pub url: String,
}

impl VisitorMessage {
    pub fn new(session_id: Option<String>, content: String) -> Self {
        Self {
            session_id: session_id
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty()),
            content: content.trim().to_string(),
        }
    }

    pub fn valid(&self) -> bool {
        !self.content.is_empty() && self.content.chars().count() <= 2000
    }
}
