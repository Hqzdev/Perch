use serde::{Deserialize, Serialize};

use crate::sources::NormalizedUrl;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSpan {
    pub url: NormalizedUrl,
    pub start_byte: usize,
    pub end_byte: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Citation {
    pub label: String,
    pub span: SourceSpan,
}
