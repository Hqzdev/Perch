use axum::{extract::State, Json};
use perch_config::ServiceSettings;
use perch_types::api::{HealthResponse, ServiceStatus};

pub async fn health_handler(State(settings): State<ServiceSettings>) -> Json<HealthResponse> {
    Json(HealthResponse {
        service: settings.name,
        status: ServiceStatus::Ok,
    })
}
