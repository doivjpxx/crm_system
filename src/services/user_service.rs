use serde::{Deserialize, Serialize};

use super::{auth_service::AuthService, claim_service};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub name: String,
    pub email: String,
    pub is_sys: Option<bool>,
}

pub struct UserService {
    pub pool: sqlx::PgPool,
}

impl UserService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user: CreateUserRequest) -> Result<(), String> {
        let password = AuthService::new().hash_password(user.password).await;

        let password = match password {
            Ok(password) => password,
            Err(e) => {
                tracing::error!("Failed to hash password: {:?}", e);
                return Err("Failed to hash password".to_string());
            }
        };

        sqlx::query!(
            r#"
            INSERT INTO users (username, email, name, password)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
            user.username,
            user.email,
            user.name,
            password
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create user: {:?}", e);
            "Failed to create user".to_string()
        })?;

        Ok(())
    }

    pub async fn get_user(&self, username: String) -> Result<UserResponse, String> {
        let user = sqlx::query!(
            r#"
            SELECT * FROM users WHERE username = $1
            "#,
            username
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get user: {:?}", e);
            "Failed to get user".to_string()
        })?;

        Ok(UserResponse {
            id: user.id,
            username: user.username,
            name: user.name,
            email: user.email,
            is_sys: None,
        })
    }

    pub async fn login(&self, username: String, password: String) -> Result<String, String> {
        let user = sqlx::query!(
            r#"
            SELECT * FROM users WHERE username = $1
            "#,
            username
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get user: {:?}", e);
            "Failed to get user".to_string()
        })?;

        let matches = AuthService::new()
            .verify_password(password, user.password)
            .await;

        if matches.is_err() {
            return Err("Invalid password".to_string());
        }

        let jwt = claim_service::Claims::encode_jwt(UserResponse {
            id: user.id,
            username: user.username,
            name: user.name,
            email: user.email,
            is_sys: None,
        })
        .map_err(|e| {
            tracing::error!("Failed to encode jwt: {:?}", e);
            "Failed to encode jwt".to_string()
        })?;

        Ok(jwt)
    }
}
