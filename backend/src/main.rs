mod api;
mod auth;
mod config;
mod db;

use config::Config;
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;

pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub config: Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let config = Config::from_env()?;
    let db_url = &config.database_url;

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = Arc::new(AppState {
        db: pool,
        config: config.clone(),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_routes = api::routes(state.clone());

    let app = axum::Router::new()
        .nest("/api", api_routes)
        .fallback_service(ServeDir::new("../frontend/dist"))
        .layer(cors)
        .with_state(state);

    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Roger Panel starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
