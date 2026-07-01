use serde::{Deserialize, Serialize};

use crate::citations::Citation;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetrievedChunk {
    pub text: String,
    pub score: f32,
    pub citation: Citation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroundedAnswer {
    pub answer: String,
    pub citations: Vec<Citation>,
}
