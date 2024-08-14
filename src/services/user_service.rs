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

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub name: String,
    pub email: String,
    pub is_sys: Option<bool>,
}

#[derive(Serialize)]
pub struct UserWithSubscriptionResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub name: String,
    pub email: String,
    pub is_sys: Option<bool>,
    pub subscription: Option<SubscriptionForUser>,
    pub resources: Vec<ResourceForUser>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SubscriptionForUser {
    pub id: uuid::Uuid,
    pub plan_id: uuid::Uuid,
    pub is_active: bool,
    pub start_date: Option<chrono::NaiveDateTime>,
    pub end_date: Option<chrono::NaiveDateTime>,
    pub trial_start_date: Option<chrono::NaiveDateTime>,
    pub trial_end_date: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct ResourceForUser {
    pub id: uuid::Uuid,
    pub name: String,
    pub max: i64,
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

        let user_subscription = sqlx::query!(
            r#"
            SELECT * FROM subscriptions WHERE user_id = $1
            "#,
            user.id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get user subscription: {:?}", e);
            "Failed to get user subscription".to_string()
        })?;

        let resources = sqlx::query!(
            r#"
            SELECT * FROM resources WHERE plan_id = $1
            "#,
            user_subscription
                .as_ref()
                .map(|s| s.plan_id)
                .unwrap_or_default()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get resources: {:?}", e);
            "Failed to get resources".to_string()
        })?;

        let jwt = claim_service::Claims::encode_jwt(UserWithSubscriptionResponse {
            id: user.id,
            username: user.username,
            name: user.name,
            email: user.email,
            is_sys: None,
            subscription: user_subscription.map(|s| SubscriptionForUser {
                id: s.id,
                plan_id: s.plan_id.unwrap_or_default(),
                is_active: s.is_active.unwrap_or_default(),
                start_date: s.start_date,
                end_date: s.end_date,
                trial_start_date: s.trial_start_date,
                trial_end_date: s.trial_end_date,
            }),
            resources: resources
                .iter()
                .map(|r| ResourceForUser {
                    id: r.id,
                    name: r.name.clone(),
                    max: r.max,
                })
                .collect(),
        })
        .map_err(|e| {
            tracing::error!("Failed to encode jwt: {:?}", e);
            "Failed to encode jwt".to_string()
        })?;

        Ok(jwt)
    }

    pub async fn change_password(
        &self,
        username: String,
        old_password: String,
        new_password: String,
    ) -> Result<(), String> {
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
            .verify_password(old_password, user.password)
            .await;

        if matches.is_err() {
            return Err("Invalid password".to_string());
        }

        let new_password = AuthService::new().hash_password(new_password).await;

        let new_password = match new_password {
            Ok(password) => password,
            Err(e) => {
                tracing::error!("Failed to hash password: {:?}", e);
                return Err("Failed to hash password".to_string());
            }
        };

        sqlx::query!(
            r#"
            UPDATE users SET password = $1 WHERE username = $2
            "#,
            new_password,
            username
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update password: {:?}", e);
            "Failed to update password".to_string()
        })?;

        Ok(())
    }
}
