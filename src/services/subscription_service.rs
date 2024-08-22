use serde::{Deserialize, Serialize};

use super::{
    plan_service::{PlanResponse, PlanService},
    user_service::{UserResponse, UserService},
};

#[derive(Deserialize)]
pub struct CreateSubscriptionRequest {
    pub user_id: uuid::Uuid,
    pub plan_id: uuid::Uuid,
}

#[derive(Serialize)]
pub struct SubscriptionResponse {
    pub id: uuid::Uuid,
    pub user_id: Option<uuid::Uuid>,
    pub plan_id: Option<uuid::Uuid>,
    pub start_date: Option<chrono::NaiveDateTime>,
    pub end_date: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize)]
pub struct SubscriptionForSysResponse {
    pub id: uuid::Uuid,
    pub user_id: Option<uuid::Uuid>,
    pub plan_id: Option<uuid::Uuid>,
    pub start_date: Option<chrono::NaiveDateTime>,
    pub end_date: Option<chrono::NaiveDateTime>,
    pub user: super::user_service::UserResponse,
    pub plan: super::plan_service::PlanResponse,
}

pub struct SubscriptionService {
    pub pool: sqlx::PgPool,
}

impl SubscriptionService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn activate_subscription(&self, subscription_id: uuid::Uuid) -> Result<(), String> {
        sqlx::query!(
            r#"
            UPDATE subscriptions
            SET is_active = true
            WHERE id = $1
            "#,
            subscription_id,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to set active subscription: {:?}", e);
            "Failed to set active subscription".to_string()
        })?;

        Ok(())
    }

    pub async fn create_subscription(
        &self,
        subscription: CreateSubscriptionRequest,
    ) -> Result<(), String> {
        let plan_service = PlanService::new(self.pool.clone());

        let plan = plan_service.get_plan(subscription.plan_id).await?;

        let start_date = chrono::Utc::now().naive_utc();
        let end_date = match plan.trial_days {
            Some(trial_days) => start_date + chrono::Duration::days(trial_days as i64),
            None => start_date + chrono::Duration::days(30),
        };

        sqlx::query!(
            r#"
            INSERT INTO subscriptions (user_id, plan_id, is_active, start_date, end_date)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            subscription.user_id,
            subscription.plan_id,
            false,
            start_date,
            end_date
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create subscription: {:?}", e);
            "Failed to create subscription".to_string()
        })?;

        Ok(())
    }

    pub async fn get_subscriptions(&self) -> Result<Vec<SubscriptionForSysResponse>, String> {
        let subscriptions = sqlx::query!(
            r#"
            SELECT s.id, user_id, plan_id, start_date, end_date, u.username, u.name as user_name, u.email, p.name as plan_name, p.price, p.trial_days, p.description
            FROM subscriptions as s
            INNER JOIN users as u ON s.user_id = u.id
            INNER JOIN plans as p ON s.plan_id = p.id
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get subscriptions: {:?}", e);
            "Failed to get subscriptions".to_string()
        })?;

        Ok(subscriptions
            .into_iter()
            .map(|subscription| SubscriptionForSysResponse {
                id: subscription.id,
                user_id: subscription.user_id,
                plan_id: subscription.plan_id,
                start_date: subscription.start_date,
                end_date: subscription.end_date,
                user: UserResponse::new(
                    subscription.user_id.unwrap_or_default(),
                    subscription.username,
                    subscription.user_name,
                    subscription.email,
                ),
                plan: PlanResponse::new(
                    subscription.plan_id.unwrap_or_default(),
                    subscription.plan_name,
                    subscription.description,
                    subscription.price,
                    None,
                    None,
                    None,
                    None,
                ),
            })
            .collect())
    }

    pub async fn get_subscription(&self, id: uuid::Uuid) -> Result<SubscriptionResponse, String> {
        let subscription = sqlx::query!(
            r#"
            SELECT id, user_id, plan_id, start_date, end_date
            FROM subscriptions
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get subscription: {:?}", e);
            "Failed to get subscription".to_string()
        })?;

        Ok(SubscriptionResponse {
            id: subscription.id,
            user_id: subscription.user_id,
            plan_id: subscription.plan_id,
            start_date: subscription.start_date,
            end_date: subscription.end_date,
        })
    }

    pub async fn get_subscriptions_for_by_username(
        &self,
        username: String,
    ) -> Result<Vec<SubscriptionResponse>, String> {
        let user_service = UserService::new(self.pool.clone());

        let user: super::user_service::UserResponse = user_service.get_user(username).await?;

        let subscriptions = sqlx::query!(
            r#"
            SELECT id, user_id, plan_id, start_date, end_date
            FROM subscriptions
            WHERE user_id = $1
            "#,
            user.id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get subscriptions: {:?}", e);
            "Failed to get subscriptions".to_string()
        })?;

        Ok(subscriptions
            .into_iter()
            .map(|subscription| SubscriptionResponse {
                id: subscription.id,
                user_id: subscription.user_id,
                plan_id: subscription.plan_id,
                start_date: subscription.start_date,
                end_date: subscription.end_date,
            })
            .collect())
    }
}
