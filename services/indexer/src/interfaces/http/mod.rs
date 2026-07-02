use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use perch_config::RuntimeSettings;
use perch_storage::Database;
use perch_types::api::{
    DependencyReadiness, DependencyStatus, ErrorBody, ErrorResponse, HealthResponse,
    IndexContentType, IndexPageRequest, IndexPageResponse, ReadinessResponse, ServiceStatus,
};

use crate::application::indexing::{IndexingService, IndexingServiceError};
use crate::domain::pages::{text_from_html, PageDocument};

#[derive(Clone)]
pub struct HttpState {
    settings: RuntimeSettings,
    database: Database,
    indexing_service: IndexingService,
}

pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: &'static str,
}

impl HttpState {
    pub fn new(
        settings: RuntimeSettings,
        database: Database,
        indexing_service: IndexingService,
    ) -> Self {
        Self {
            settings,
            database,
            indexing_service,
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

pub async fn index_page_handler(
    State(state): State<HttpState>,
    Json(request): Json<IndexPageRequest>,
) -> Result<(StatusCode, Json<IndexPageResponse>), ApiError> {
    let content = match request.content_type {
        IndexContentType::Html => text_from_html(&request.content),
        IndexContentType::Text => request.content,
    };
    let page = state
        .indexing_service
        .index_page(PageDocument::new(
            request.site_id,
            request.url,
            request.title,
            content,
        ))
        .await
        .map_err(api_error_from_indexing_error)?;

    Ok((
        StatusCode::CREATED,
        Json(IndexPageResponse {
            page_id: page.page_id,
            chunks_indexed: page.chunks_indexed,
        }),
    ))
}

fn api_error_from_indexing_error(error: IndexingServiceError) -> ApiError {
    match error {
        IndexingServiceError::InvalidPage => ApiError::new(
            StatusCode::BAD_REQUEST,
            "invalid_page",
            "The page payload is invalid.",
        ),
        IndexingServiceError::Storage(_) => ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "storage_error",
            "The page could not be indexed.",
        ),
    }
}
