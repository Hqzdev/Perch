use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageDocument {
    pub site_id: Uuid,
    pub url: String,
    pub title: Option<String>,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedPage {
    pub page_id: Uuid,
    pub chunks_indexed: usize,
    pub chunks: Vec<IndexedChunk>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedChunk {
    pub chunk_id: Uuid,
    pub page_id: Uuid,
    pub chunk_index: usize,
    pub content: String,
    pub source_url: String,
    pub source_title: String,
}

impl PageDocument {
    pub fn new(site_id: Uuid, url: String, title: Option<String>, content: String) -> Self {
        Self {
            site_id,
            url: url.trim().trim_end_matches('/').to_string(),
            title: title
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty()),
            content: normalize_whitespace(&content),
        }
    }

    pub fn valid(&self) -> bool {
        !self.url.is_empty()
            && (self.url.starts_with("http://") || self.url.starts_with("https://"))
            && !self.content.is_empty()
            && self.content.chars().count() <= 200_000
    }
}

pub fn text_from_html(value: &str) -> String {
    let mut text = String::with_capacity(value.len());
    let mut in_tag = false;

    for character in value.chars() {
        match character {
            '<' => {
                in_tag = true;
                text.push(' ');
            }
            '>' => {
                in_tag = false;
                text.push(' ');
            }
            _ if !in_tag => text.push(character),
            _ => {}
        }
    }

    normalize_whitespace(&text)
}

fn normalize_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}
