mod application;
mod config;
mod domain;
mod infrastructure;
mod interfaces;

use anyhow::Context;
use axum::Router;
use perch_config::ServiceSettings;
use tower_http::trace::TraceLayer;

use crate::interfaces::http::health_handler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let settings = ServiceSettings::from_env("gateway", 8080)?;
    let app = Router::new()
        .route("/health", axum::routing::get(health_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(settings.clone());

    let listener = tokio::net::TcpListener::bind(settings.bind_addr)
        .await
        .with_context(|| format!("failed to bind {}", settings.bind_addr))?;

    tracing::info!(service = settings.name, bind_addr = %settings.bind_addr, "service started");
    axum::serve(listener, app).await?;

    Ok(())
}
