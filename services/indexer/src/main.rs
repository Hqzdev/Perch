mod application;
mod config;
mod domain;
mod infrastructure;
mod interfaces;

use anyhow::Context;
use axum::Router;
use perch_config::RuntimeSettings;
use perch_storage::Database;
use tower_http::trace::TraceLayer;

use crate::application::indexing::IndexingService;
use crate::infrastructure::crawler::WebCrawler;
use crate::infrastructure::storage::PageRepository;
use crate::interfaces::http::{
    crawl_job_handler, health_handler, index_page_handler, readiness_handler, HttpState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let settings = RuntimeSettings::from_env("indexer", 8081)?;
    let database = Database::new(&settings.data_stores.database_url)?;
    let indexing_service =
        IndexingService::new(WebCrawler::new(), PageRepository::new(database.clone()));
    let state = HttpState::new(settings.clone(), database, indexing_service);
    let app = Router::new()
        .route("/health", axum::routing::get(health_handler))
        .route("/ready", axum::routing::get(readiness_handler))
        .route("/v1/index/pages", axum::routing::post(index_page_handler))
        .route("/v1/crawl/jobs", axum::routing::post(crawl_job_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(settings.service.bind_addr)
        .await
        .with_context(|| format!("failed to bind {}", settings.service.bind_addr))?;

    tracing::info!(service = settings.service.name, bind_addr = %settings.service.bind_addr, "service started");
    axum::serve(listener, app).await?;

    Ok(())
}
