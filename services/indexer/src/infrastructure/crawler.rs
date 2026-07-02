use thiserror::Error;

#[derive(Debug, Clone)]
pub struct WebCrawler {
    client: reqwest::Client,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FetchedPage {
    pub url: String,
    pub title: Option<String>,
    pub html: String,
}

#[derive(Debug, Error)]
pub enum WebCrawlerError {
    #[error("page fetch failed: {0}")]
    Request(#[from] reqwest::Error),
}

impl WebCrawler {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch(&self, url: &str) -> Result<FetchedPage, WebCrawlerError> {
        let response = self
            .client
            .get(url)
            .header(reqwest::header::USER_AGENT, "PerchIndexer/0.1")
            .send()
            .await?
            .error_for_status()?;
        let final_url = response.url().to_string();
        let html = response.text().await?;

        Ok(FetchedPage {
            title: title_from_html(&html),
            url: final_url,
            html,
        })
    }
}

fn title_from_html(html: &str) -> Option<String> {
    let lower = html.to_lowercase();
    let start = lower.find("<title>")? + "<title>".len();
    let end = lower[start..].find("</title>")? + start;
    let title = html[start..end]
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    if title.is_empty() {
        None
    } else {
        Some(title)
    }
}
