pub mod auth;
pub mod servers;

use axum::{routing::get, Router};
use std::sync::Arc;

use crate::AppState;

pub fn routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/servers", servers::routes(state))
        .route("/health", get(health))
}

async fn health() -> &'static str {
    "ok"
}
