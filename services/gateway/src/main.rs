mod application;
mod config;
mod domain;
mod infrastructure;
mod interfaces;

use anyhow::Context;
use axum::Router;
use perch_config::RuntimeSettings;
use perch_storage::Database;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::application::sites::SiteService;
use crate::infrastructure::storage::SiteRepository;
use crate::interfaces::http::{
    create_site_handler, health_handler, readiness_handler, widget_config_handler, HttpState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let settings = RuntimeSettings::from_env("gateway", 8080)?;
    let database = Database::new(&settings.data_stores.database_url)?;
    let site_service = SiteService::new(SiteRepository::new(database.clone()));
    let state = HttpState::new(settings.clone(), database, site_service);
    let app = Router::new()
        .route("/health", axum::routing::get(health_handler))
        .route("/ready", axum::routing::get(readiness_handler))
        .route("/v1/sites", axum::routing::post(create_site_handler))
        .route(
            "/v1/widget/config",
            axum::routing::get(widget_config_handler),
        )
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(settings.service.bind_addr)
        .await
        .with_context(|| format!("failed to bind {}", settings.service.bind_addr))?;

    tracing::info!(service = settings.service.name, bind_addr = %settings.service.bind_addr, "service started");
    axum::serve(listener, app).await?;

    Ok(())
}
