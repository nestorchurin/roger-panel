use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::auth::jwt::verify_token;
use crate::AppState;

#[derive(Clone, Debug)]
pub struct AuthUser {
    #[allow(dead_code)]
    pub user_id: String,
}

pub async fn auth_middleware(
    State(state): State<std::sync::Arc<AppState>>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let header = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = verify_token(token, &state.config.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let auth_user = AuthUser {
        user_id: claims.sub,
    };

    request.extensions_mut().insert(auth_user);

    Ok(next.run(request).await)
}
