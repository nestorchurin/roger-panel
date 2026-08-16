use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use serde_json::{json, Value};

use crate::auth::middleware::auth_middleware;
use crate::db::models::{CreateServer, Server};
use crate::db::queries;
use crate::AppState;

pub fn routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_servers).post(create_server))
        .layer(middleware::from_fn_with_state(state, auth_middleware))
}

async fn list_servers(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Server>>, (StatusCode, Json<Value>)> {
    let servers = queries::list_servers(&state.db)
        .await
        .map_err(err)?;

    Ok(Json(servers))
}

async fn create_server(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateServer>,
) -> Result<Json<Server>, (StatusCode, Json<Value>)> {
    let server = queries::create_server(&state.db, &input)
        .await
        .map_err(err)?;

    Ok(Json(server))
}

fn err(e: anyhow::Error) -> (StatusCode, Json<Value>) {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
}
