use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use std::sync::Arc;
use serde_json::{json, Value};

use crate::auth::jwt::create_token;
use crate::db::models::{AuthResponse, CreateUser, LoginUser};
use crate::db::queries;
use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateUser>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<Value>)> {
    let existing = queries::find_user_by_email(&state.db, &input.email)
        .await
        .map_err(err)?;

    if existing.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({"error": "User with this email already exists"})),
        ));
    }

    let salt = password_hash::SaltString::generate(&mut rand::thread_rng());
    let password_hash = argon2::password_hash::PasswordHash::generate(
        argon2::Argon2::default(),
        input.password.as_bytes(),
        &salt,
    )
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .to_string();

    let user = queries::create_user(&state.db, &input, &password_hash)
        .await
        .map_err(err)?;

    let token = create_token(&user.id, &state.config.jwt_secret, state.config.jwt_expires_in)
        .map_err(err)?;

    Ok(Json(AuthResponse { token, user }))
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(input): Json<LoginUser>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<Value>)> {
    let user = queries::find_user_by_email(&state.db, &input.email)
        .await
        .map_err(err)?
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid credentials"}))))?;

    let password_hash = argon2::password_hash::PasswordHash::new(&user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    password_hash
        .verify_password(&[&argon2::Argon2::default()], input.password.as_bytes())
        .map_err(|_| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid credentials"}))))?;

    let token = create_token(&user.id, &state.config.jwt_secret, state.config.jwt_expires_in)
        .map_err(err)?;

    Ok(Json(AuthResponse { token, user }))
}

fn err(e: anyhow::Error) -> (StatusCode, Json<Value>) {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
}
