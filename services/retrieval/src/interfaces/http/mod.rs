use axum::{extract::State, http::StatusCode, Json};
use perch_config::RuntimeSettings;
use perch_storage::Database;
use perch_types::api::{
    DependencyReadiness, DependencyStatus, HealthResponse, ReadinessResponse,
    RetrievalAnswerRequest, RetrievalAnswerResponse, ServiceStatus,
};

use crate::application::answering::AnswerService;

#[derive(Clone)]
pub struct HttpState {
    settings: RuntimeSettings,
    database: Database,
    answer_service: AnswerService,
}

impl HttpState {
    pub fn new(
        settings: RuntimeSettings,
        database: Database,
        answer_service: AnswerService,
    ) -> Self {
        Self {
            settings,
            database,
            answer_service,
        }
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
    State(state): State<HttpState>,
    Json(request): Json<RetrievalAnswerRequest>,
) -> (StatusCode, Json<RetrievalAnswerResponse>) {
    match state.answer_service.answer(request).await {
        Ok(response) => (StatusCode::OK, Json(response)),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RetrievalAnswerResponse {
                answer: "Retrieval failed while searching indexed content.".to_string(),
                citations: Vec::new(),
            }),
        ),
    }
}
