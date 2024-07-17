use serde::{Deserialize, Serialize};

use super::auth_service::AuthService;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
}

pub struct UserService {
    pub pool: sqlx::PgPool,
}

impl UserService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user: CreateUserRequest) -> Result<(), String> {
        let password = AuthService::hash_password(user.password).await;

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
            email: user.email,
        })
    }
}
