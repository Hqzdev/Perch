use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use perch_config::RuntimeSettings;
use perch_storage::Database;
use perch_types::api::{
    CreateSiteRequest, DependencyReadiness, DependencyStatus, ErrorBody, ErrorResponse,
    HealthResponse, ReadinessResponse, ServiceStatus, SiteResponse, WidgetChatRequest,
    WidgetChatResponse, WidgetCitation, WidgetConfigResponse, WidgetFeatures, WidgetTheme,
};
use serde::Deserialize;

use crate::application::sites::{SiteService, SiteServiceError};
use crate::domain::messages::{AssistantAnswer, VisitorMessage};
use crate::domain::sites::{NewSite, Site};

#[derive(Clone)]
pub struct HttpState {
    settings: RuntimeSettings,
    database: Database,
    site_service: SiteService,
}

#[derive(Debug, Deserialize)]
pub struct WidgetConfigQuery {
    key: String,
}

pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: &'static str,
}

impl HttpState {
    pub fn new(settings: RuntimeSettings, database: Database, site_service: SiteService) -> Self {
        Self {
            settings,
            database,
            site_service,
        }
    }
}

impl ApiError {
    fn new(status: StatusCode, code: &'static str, message: &'static str) -> Self {
        Self {
            status,
            code,
            message,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(ErrorResponse {
                error: ErrorBody {
                    code: self.code.to_string(),
                    message: self.message.to_string(),
                },
            }),
        )
            .into_response()
    }
}

pub async fn health_handler(State(state): State<HttpState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        service: state.settings.service.name,
        status: ServiceStatus::Ok,
    })
}

pub async fn readiness_handler(
    State(state): State<HttpState>,
) -> (StatusCode, Json<ReadinessResponse>) {
    let postgres_status = match state.database.ready().await {
        Ok(()) => DependencyStatus::Ok,
        Err(_) => DependencyStatus::Unavailable,
    };
    let service_status = if postgres_status == DependencyStatus::Ok {
        ServiceStatus::Ok
    } else {
        ServiceStatus::Unavailable
    };
    let status_code = if service_status == ServiceStatus::Ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (
        status_code,
        Json(ReadinessResponse {
            service: state.settings.service.name,
            status: service_status,
            environment: state.settings.environment,
            dependencies: vec![
                DependencyReadiness {
                    name: "postgres".to_string(),
                    status: postgres_status,
                },
                DependencyReadiness {
                    name: "redis".to_string(),
                    status: DependencyStatus::Configured,
                },
                DependencyReadiness {
                    name: "qdrant".to_string(),
                    status: DependencyStatus::Configured,
                },
            ],
        }),
    )
}

pub async fn create_site_handler(
    State(state): State<HttpState>,
    Json(request): Json<CreateSiteRequest>,
) -> Result<(StatusCode, Json<SiteResponse>), ApiError> {
    let site = state
        .site_service
        .create_site(NewSite::new(
            request.organization_name,
            request.site_name,
            request.origin,
        ))
        .await
        .map_err(api_error_from_site_error)?;

    Ok((StatusCode::CREATED, Json(site_response(site))))
}

pub async fn widget_config_handler(
    State(state): State<HttpState>,
    Query(query): Query<WidgetConfigQuery>,
    headers: HeaderMap,
) -> Result<Json<WidgetConfigResponse>, ApiError> {
    let origin = headers.get("origin").and_then(|value| value.to_str().ok());
    let site = state
        .site_service
        .resolve_widget_site(&query.key, origin)
        .await
        .map_err(api_error_from_site_error)?;

    Ok(Json(WidgetConfigResponse {
        site_name: site.name,
        theme: WidgetTheme {
            accent_color: "#12b76a".to_string(),
            placement: "bottom-right".to_string(),
        },
        features: WidgetFeatures {
            citations: true,
            streaming: true,
        },
    }))
}

pub async fn widget_chat_handler(
    State(state): State<HttpState>,
    headers: HeaderMap,
    Json(request): Json<WidgetChatRequest>,
) -> Result<Json<WidgetChatResponse>, ApiError> {
    let origin = headers.get("origin").and_then(|value| value.to_str().ok());
    let answer = state
        .site_service
        .answer_widget_message(
            &request.public_key,
            origin,
            VisitorMessage::new(request.session_id, request.message),
        )
        .await
        .map_err(api_error_from_site_error)?;

    Ok(Json(widget_chat_response(answer)))
}

fn site_response(site: Site) -> SiteResponse {
    SiteResponse {
        id: site.id,
        organization_id: site.organization_id,
        name: site.name,
        origin: site.origin,
        script_key: site.script_key,
    }
}

fn widget_chat_response(answer: AssistantAnswer) -> WidgetChatResponse {
    WidgetChatResponse {
        conversation_id: answer.conversation_id,
        message_id: answer.message_id,
        answer: answer.content,
        citations: answer
            .citations
            .into_iter()
            .map(|citation| WidgetCitation {
                title: citation.title,
                url: citation.url,
            })
            .collect(),
    }
}

fn api_error_from_site_error(error: SiteServiceError) -> ApiError {
    match error {
        SiteServiceError::InvalidSite => ApiError::new(
            StatusCode::BAD_REQUEST,
            "invalid_site",
            "The site payload is invalid.",
        ),
        SiteServiceError::InvalidMessage => ApiError::new(
            StatusCode::BAD_REQUEST,
            "invalid_message",
            "The message payload is invalid.",
        ),
        SiteServiceError::MissingOrigin => ApiError::new(
            StatusCode::BAD_REQUEST,
            "missing_origin",
            "The Origin header is required.",
        ),
        SiteServiceError::OriginNotAllowed => ApiError::new(
            StatusCode::FORBIDDEN,
            "domain_not_allowed",
            "This widget key is not allowed on the current domain.",
        ),
        SiteServiceError::NotFound => ApiError::new(
            StatusCode::NOT_FOUND,
            "site_not_found",
            "The widget site was not found.",
        ),
        SiteServiceError::Storage(_) => ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "storage_error",
            "The request could not be completed.",
        ),
    }
}
