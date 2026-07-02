use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingVector {
    pub values: Vec<f32>,
}

impl EmbeddingVector {
    pub fn dimensions(&self) -> usize {
        self.values.len()
    }
}

pub const HASH_EMBEDDING_DIMENSIONS: usize = 64;

pub fn hash_embedding(text: &str) -> EmbeddingVector {
    let mut values = vec![0.0; HASH_EMBEDDING_DIMENSIONS];

    for token in text
        .split(|character: char| !character.is_alphanumeric())
        .filter(|token| !token.is_empty())
    {
        let hash = token_hash(&token.to_lowercase());
        let index = hash as usize % HASH_EMBEDDING_DIMENSIONS;
        let sign = if hash & 1 == 0 { 1.0 } else { -1.0 };
        values[index] += sign;
    }

    normalize(values)
}

fn normalize(mut values: Vec<f32>) -> EmbeddingVector {
    let magnitude = values.iter().map(|value| value * value).sum::<f32>().sqrt();

    if magnitude > 0.0 {
        for value in &mut values {
            *value /= magnitude;
        }
    }

    EmbeddingVector { values }
}

fn token_hash(value: &str) -> u64 {
    value.bytes().fold(0xcbf29ce484222325u64, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
    })
}
