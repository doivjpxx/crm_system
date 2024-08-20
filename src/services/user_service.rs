use serde::{Deserialize, Serialize};

use super::{auth_service::AuthService, claim_service};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub name: Option<String>,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub username: String,
    pub email: String,
    pub name: Option<String>,
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
    pub created_at: chrono::NaiveDateTime,
}

impl UserResponse {
    pub fn new(id: uuid::Uuid, username: String, name: String, email: String) -> Self {
        Self {
            id,
            username,
            name,
            email,
            is_sys: None,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
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

    pub async fn get_users(&self) -> Result<Vec<UserResponse>, String> {
        let users = sqlx::query!(
            r#"
            SELECT * FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get users: {:?}", e);
            "Failed to get users".to_string()
        })?;

        Ok(users
            .iter()
            .map(|user| UserResponse {
                id: user.id,
                username: user.username.clone(),
                name: user.name.clone(),
                email: user.email.clone(),
                is_sys: None,
                created_at: user.created_at.unwrap(),
            })
            .collect())
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
            user.name.unwrap_or_default(),
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
            created_at: user.created_at.unwrap(),
            is_sys: None,
        })
    }

    pub async fn login(
        &self,
        username: String,
        password: String,
    ) -> Result<(String, String), String> {
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

        let exist_token = sqlx::query!(
            r#"
            SELECT * FROM tokens WHERE user_id = $1
            "#,
            user.id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get token: {:?}", e);
            "Failed to get token".to_string()
        })?;

        if exist_token.is_some() {
            sqlx::query!(
                r#"
                DELETE FROM tokens WHERE user_id = $1
                "#,
                user.id
            )
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete token: {:?}", e);
                "Failed to delete token".to_string()
            })?;
        }

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
            username: user.username.clone(),
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

        let refresh_token =
            claim_service::Claims::encode_refresh_jwt(user.username).map_err(|e| {
                tracing::error!("Failed to encode refresh jwt: {:?}", e);
                "Failed to encode refresh jwt".to_string()
            })?;

        sqlx::query!(
            r#"
            INSERT INTO tokens (user_id, token)
            VALUES ($1, $2)
            "#,
            user.id,
            refresh_token
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to insert token: {:?}", e);
            "Failed to insert token".to_string()
        })?;

        Ok((jwt, refresh_token))
    }

    pub async fn refresh_token(&self, refresh_token: String) -> Result<String, String> {
        let token = sqlx::query!(
            r#"
            SELECT * FROM tokens WHERE token = $1
            "#,
            refresh_token
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get token: {:?}", e);
            "Failed to get token".to_string()
        })?;

        let user = sqlx::query!(
            r#"
            SELECT * FROM users WHERE id = $1
            "#,
            token.user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get user: {:?}", e);
            "Failed to get user".to_string()
        })?;

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
            username: user.username.clone(),
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

    pub async fn update_user(
        &self,
        username: String,
        user: UpdateUserRequest,
    ) -> Result<(), String> {
        let exist_user = sqlx::query!(
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

        if exist_user.username != user.username {
            let exist_user = sqlx::query!(
                r#"
                SELECT * FROM users WHERE username = $1
                "#,
                user.username
            )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get user: {:?}", e);
                "Failed to get user".to_string()
            })?;

            if exist_user.is_some() {
                return Err("Username already exists".to_string());
            }
        }

        sqlx::query!(
            r#"
            UPDATE users SET email = $1, name = $2 WHERE username = $3
            "#,
            user.email,
            user.name.unwrap_or_default(),
            user.username,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update user: {:?}", e);
            "Failed to update user".to_string()
        })?;

        Ok(())
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
