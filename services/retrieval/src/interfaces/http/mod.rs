use axum::{extract::State, http::StatusCode, Json};
use perch_config::RuntimeSettings;
use perch_storage::Database;
use perch_types::api::{
    DependencyReadiness, DependencyStatus, HealthResponse, ReadinessResponse,
    RetrievalAnswerRequest, RetrievalAnswerResponse, ServiceStatus, WidgetCitation,
};

#[derive(Clone)]
pub struct HttpState {
    settings: RuntimeSettings,
    database: Database,
}

impl HttpState {
    pub fn new(settings: RuntimeSettings, database: Database) -> Self {
        Self { settings, database }
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

pub async fn answer_handler(
    Json(request): Json<RetrievalAnswerRequest>,
) -> (StatusCode, Json<RetrievalAnswerResponse>) {
    let answer = format!(
        "Perch searched the currently indexed context for {}. Retrieval is wired through the dedicated retrieval service; the next step is replacing this bootstrap answer with tenant-filtered chunks from Qdrant and Postgres. Question: {}",
        request.site_name,
        request.question.trim()
    );

    (
        StatusCode::OK,
        Json(RetrievalAnswerResponse {
            answer,
            citations: vec![WidgetCitation {
                title: request.site_name,
                url: request.site_origin,
            }],
        }),
    )
}
