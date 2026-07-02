use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthResponse {
    pub service: String,
    pub status: ServiceStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessResponse {
    pub service: String,
    pub status: ServiceStatus,
    pub environment: String,
    pub dependencies: Vec<DependencyReadiness>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyReadiness {
    pub name: String,
    pub status: DependencyStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceStatus {
    Ok,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyStatus {
    Configured,
    Ok,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateSiteRequest {
    pub organization_name: String,
    pub site_name: String,
    pub origin: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SiteResponse {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub origin: String,
    pub script_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardSiteSummary {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub origin: String,
    pub script_key: String,
    pub pages_indexed: usize,
    pub conversations_count: usize,
    pub last_indexed_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardSiteDetail {
    pub site: DashboardSiteSummary,
    pub install_snippet: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardPageSummary {
    pub id: Uuid,
    pub url: String,
    pub title: Option<String>,
    pub status: String,
    pub chunks_indexed: usize,
    pub last_indexed_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardConversationSummary {
    pub id: Uuid,
    pub visitor_id: Option<String>,
    pub messages_count: usize,
    pub last_message_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WidgetConfigResponse {
    pub site_name: String,
    pub theme: WidgetTheme,
    pub features: WidgetFeatures,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WidgetChatRequest {
    pub public_key: String,
    pub session_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WidgetChatResponse {
    pub conversation_id: Uuid,
    pub message_id: Uuid,
    pub answer: String,
    pub citations: Vec<WidgetCitation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WidgetCitation {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalAnswerRequest {
    pub site_id: Uuid,
    pub site_name: String,
    pub site_origin: String,
    pub question: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalAnswerResponse {
    pub answer: String,
    pub citations: Vec<WidgetCitation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexPageRequest {
    pub site_id: Uuid,
    pub url: String,
    pub title: Option<String>,
    pub content: String,
    pub content_type: IndexContentType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexSitePageRequest {
    pub url: String,
    pub title: Option<String>,
    pub content: String,
    pub content_type: IndexContentType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrawlSiteRequest {
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrawlJobRequest {
    pub site_id: Uuid,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndexContentType {
    Html,
    Text,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexPageResponse {
    pub page_id: Uuid,
    pub chunks_indexed: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrawlJobResponse {
    pub job_id: Uuid,
    pub site_id: Uuid,
    pub url: String,
    pub status: CrawlJobStatus,
    pub page_id: Option<Uuid>,
    pub pages_indexed: usize,
    pub chunks_indexed: usize,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrawlJobStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WidgetTheme {
    pub accent_color: String,
    pub placement: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WidgetFeatures {
    pub citations: bool,
    pub streaming: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorBody,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
}
