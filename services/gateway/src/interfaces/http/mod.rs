use axum::{extract::State, Json};
use perch_config::RuntimeSettings;
use perch_types::api::{
    DependencyReadiness, DependencyStatus, HealthResponse, ReadinessResponse, ServiceStatus,
};

pub async fn health_handler(State(settings): State<RuntimeSettings>) -> Json<HealthResponse> {
    Json(HealthResponse {
        service: settings.service.name,
        status: ServiceStatus::Ok,
    })
}

pub async fn readiness_handler(State(settings): State<RuntimeSettings>) -> Json<ReadinessResponse> {
    Json(ReadinessResponse {
        service: settings.service.name,
        status: ServiceStatus::Ok,
        environment: settings.environment,
        dependencies: vec![
            DependencyReadiness {
                name: "postgres".to_string(),
                status: DependencyStatus::Configured,
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
    })
}
