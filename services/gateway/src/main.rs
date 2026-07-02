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
use crate::infrastructure::indexer::IndexerClient;
use crate::infrastructure::retrieval::RetrievalClient;
use crate::infrastructure::storage::SiteRepository;
use crate::interfaces::http::{
    crawl_job_status_handler, crawl_site_page_handler, create_site_handler, health_handler,
    index_site_page_handler, list_site_conversations_handler, list_site_pages_handler,
    list_sites_handler, readiness_handler, site_detail_handler, widget_chat_handler,
    widget_config_handler, HttpState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let settings = RuntimeSettings::from_env("gateway", 8080)?;
    let database = Database::new(&settings.data_stores.database_url)?;
    let indexer = IndexerClient::new(settings.services.indexer_url.clone());
    let retrieval = RetrievalClient::new(settings.services.retrieval_url.clone());
    let site_service = SiteService::new(SiteRepository::new(database.clone()), indexer, retrieval);
    let state = HttpState::new(settings.clone(), database, site_service);
    let app = Router::new()
        .route("/health", axum::routing::get(health_handler))
        .route("/ready", axum::routing::get(readiness_handler))
        .route(
            "/v1/sites",
            axum::routing::get(list_sites_handler).post(create_site_handler),
        )
        .route(
            "/v1/sites/{site_id}",
            axum::routing::get(site_detail_handler),
        )
        .route(
            "/v1/sites/{site_id}/pages",
            axum::routing::get(list_site_pages_handler).post(index_site_page_handler),
        )
        .route(
            "/v1/sites/{site_id}/conversations",
            axum::routing::get(list_site_conversations_handler),
        )
        .route(
            "/v1/sites/{site_id}/crawl-jobs",
            axum::routing::post(crawl_site_page_handler),
        )
        .route(
            "/v1/sites/{site_id}/crawl-jobs/{job_id}",
            axum::routing::get(crawl_job_status_handler),
        )
        .route(
            "/v1/widget/config",
            axum::routing::get(widget_config_handler),
        )
        .route("/v1/widget/chat", axum::routing::post(widget_chat_handler))
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
