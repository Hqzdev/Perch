#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceChunk {
    pub content: String,
    pub source_url: String,
    pub source_title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrievedContext {
    pub chunks: Vec<SourceChunk>,
}

impl RetrievedContext {
    pub fn empty() -> Self {
        Self { chunks: Vec::new() }
    }

    pub fn has_sources(&self) -> bool {
        !self.chunks.is_empty()
    }
}
