use serde::{Deserialize, Serialize};

use super::{auth_service::AuthService, claim_service};

#[derive(Deserialize)]
pub struct SysLoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct SysResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub name: String,
}

pub struct SysService {
    pub pool: sqlx::PgPool,
}

impl SysService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_sys(&self, username: String) -> Result<SysResponse, String> {
        let sys = sqlx::query!(
            r#"
            SELECT id, username, name
            FROM sys
            WHERE username = $1
            "#,
            username
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get sys: {:?}", e);
            "Failed to get sys".to_string()
        })?;

        Ok(SysResponse {
            id: sys.id,
            username: sys.username,
            name: sys.name,
        })
    }

    pub async fn login(&self, user: SysLoginRequest) -> Result<String, String> {
        let sys = sqlx::query!(
            r#"
            SELECT id, username, name, password
            FROM sys
            WHERE username = $1
            "#,
            user.username
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get sys: {:?}", e);
            "Failed to get sys".to_string()
        })?;

        let matches = AuthService::new()
            .verify_password(user.password, sys.password)
            .await;

        if matches.is_err() {
            return Err("Invalid password".to_string());
        }

        let jwt = claim_service::Claims::encode_jwt_sys(SysResponse {
            id: sys.id,
            username: sys.username,
            name: sys.name,
        })
        .map_err(|e| {
            tracing::error!("Failed to encode jwt: {:?}", e);
            "Failed to encode jwt".to_string()
        })?;

        Ok(jwt)
    }
}
