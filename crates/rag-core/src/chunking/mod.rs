use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextChunk {
    pub index: usize,
    pub text: String,
    pub start_byte: usize,
    pub end_byte: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkingConfig {
    pub max_chars: usize,
}

impl ChunkingConfig {
    pub fn conservative() -> Self {
        Self { max_chars: 1_200 }
    }
}

pub fn chunk_text(text: &str, config: ChunkingConfig) -> Vec<TextChunk> {
    if text.is_empty() {
        return Vec::new();
    }

    let mut chunks = Vec::new();
    let mut start = 0;
    let mut index = 0;

    while start < text.len() {
        let mut end = usize::min(start + config.max_chars, text.len());

        while end < text.len() && !text.is_char_boundary(end) {
            end -= 1;
        }

        if end == start {
            break;
        }

        chunks.push(TextChunk {
            index,
            text: text[start..end].trim().to_owned(),
            start_byte: start,
            end_byte: end,
        });

        start = end;
        index += 1;
    }

    chunks
}
