use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub server_type: String,
    pub version: String,
    pub port: i64,
    pub max_ram_mb: i64,
    pub min_ram_mb: i64,
    pub cpu_limit: Option<f64>,
    pub iops_limit: Option<i64>,
    pub net_rx_limit: Option<i64>,
    pub net_tx_limit: Option<i64>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateServer {
    pub name: String,
    pub server_type: String,
    pub version: String,
    pub port: Option<i64>,
    pub max_ram_mb: Option<i64>,
    pub min_ram_mb: Option<i64>,
    pub cpu_limit: Option<f64>,
    pub iops_limit: Option<i64>,
    pub net_rx_limit: Option<i64>,
    pub net_tx_limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub expires_at: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Setting {
    pub key: String,
    pub value: String,
    pub updated_at: String,
}
