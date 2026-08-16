use super::models::*;
use anyhow::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

fn now() -> String {
    chrono::Utc::now().to_rfc3339()
}

pub async fn create_user(pool: &SqlitePool, user: &CreateUser, password_hash: &str) -> Result<User> {
    let id = Uuid::new_v4().to_string();
    let time = now();

    let user = sqlx::query_as::<_, User>(
        r#"INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at)
           VALUES (?, ?, ?, ?, 'admin', ?, ?)
           RETURNING *"#,
    )
    .bind(&id)
    .bind(&user.username)
    .bind(&user.email)
    .bind(password_hash)
    .bind(&time)
    .bind(&time)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn find_user_by_email(pool: &SqlitePool, email: &str) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn find_user_by_id(pool: &SqlitePool, id: &str) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn user_exists(pool: &SqlitePool) -> Result<bool> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    Ok(count.0 > 0)
}

pub async fn create_server(pool: &SqlitePool, data: &CreateServer) -> Result<Server> {
    let id = Uuid::new_v4().to_string();
    let time = now();
    let port = data.port.unwrap_or(25565);
    let max_ram = data.max_ram_mb.unwrap_or(2048);
    let min_ram = data.min_ram_mb.unwrap_or(512);

    let server = sqlx::query_as::<_, Server>(
        r#"INSERT INTO servers (id, name, server_type, version, port, max_ram_mb, min_ram_mb,
                                cpu_limit, iops_limit, net_rx_limit, net_tx_limit, status, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'stopped', ?, ?)
           RETURNING *"#,
    )
    .bind(&id)
    .bind(&data.name)
    .bind(&data.server_type)
    .bind(&data.version)
    .bind(port)
    .bind(max_ram)
    .bind(min_ram)
    .bind(data.cpu_limit)
    .bind(data.iops_limit)
    .bind(data.net_rx_limit)
    .bind(data.net_tx_limit)
    .bind(&time)
    .bind(&time)
    .fetch_one(pool)
    .await?;

    Ok(server)
}

pub async fn list_servers(pool: &SqlitePool) -> Result<Vec<Server>> {
    let servers = sqlx::query_as::<_, Server>("SELECT * FROM servers ORDER BY created_at DESC")
        .fetch_all(pool)
        .await?;

    Ok(servers)
}
