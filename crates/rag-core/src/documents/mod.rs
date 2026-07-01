use serde::{Deserialize, Serialize};

use crate::sources::NormalizedUrl;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebPage {
    pub url: NormalizedUrl,
    pub title: Option<String>,
    pub text: String,
}
