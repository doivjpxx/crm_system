use crate::domain::dtos::{
    plan_dtos::PlanResponse,
    subscription_dtos::{
        CreateSubscriptionRequest, SubscriptionForSysResponse, SubscriptionResponse,
    },
    user_dtos::UserResponse,
};

use super::{
    plan_service::{PlanService, PlanServiceImpl},
    user_service::{UserService, UserServiceImpl},
};

pub struct SubscriptionService {
    pub pool: sqlx::PgPool,
}

pub trait SubscriptionServiceImpl {
    fn new(pool: sqlx::PgPool) -> Self;

    async fn activate_subscription(&self, subscription_id: uuid::Uuid) -> Result<(), String>;

    async fn deactivate_subscription(&self, subscription_id: uuid::Uuid) -> Result<(), String>;

    async fn create_subscription(
        &self,
        subscription: CreateSubscriptionRequest,
    ) -> Result<SubscriptionResponse, String>;

    async fn get_subscriptions(&self) -> Result<Vec<SubscriptionForSysResponse>, String>;

    async fn get_subscription(&self, id: uuid::Uuid) -> Result<SubscriptionResponse, String>;

    async fn get_subscriptions_for_by_username(
        &self,
        username: String,
    ) -> Result<Vec<SubscriptionResponse>, String>;
}

impl SubscriptionServiceImpl for SubscriptionService {
    fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    async fn activate_subscription(&self, subscription_id: uuid::Uuid) -> Result<(), String> {
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

    async fn deactivate_subscription(&self, subscription_id: uuid::Uuid) -> Result<(), String> {
        sqlx::query!(
            r#"
            UPDATE subscriptions
            SET is_active = false
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

    async fn create_subscription(
        &self,
        subscription: CreateSubscriptionRequest,
    ) -> Result<SubscriptionResponse, String> {
        let plan_service = PlanService::new(self.pool.clone());

        let plan = plan_service.get_plan(subscription.plan_id).await?;

        let start_date = chrono::Utc::now().naive_utc();
        let end_date = match plan.trial_days {
            Some(trial_days) => start_date + chrono::Duration::days(trial_days as i64),
            None => start_date + chrono::Duration::days(30),
        };

        let sub = sqlx::query!(
            r#"
            INSERT INTO subscriptions (user_id, plan_id, is_active, start_date, end_date)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, plan_id, start_date, end_date
            "#,
            subscription.user_id,
            subscription.plan_id,
            false,
            start_date,
            end_date
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create subscription: {:?}", e);
            "Failed to create subscription".to_string()
        })?;

        Ok(SubscriptionResponse {
            id: sub.id,
            user_id: sub.user_id,
            plan_id: sub.plan_id,
            start_date: sub.start_date,
            end_date: sub.end_date,
            is_active: false,
        })
    }

    async fn get_subscriptions(&self) -> Result<Vec<SubscriptionForSysResponse>, String> {
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

    async fn get_subscription(&self, id: uuid::Uuid) -> Result<SubscriptionResponse, String> {
        let subscription = sqlx::query!(
            r#"
            SELECT id, user_id, plan_id, start_date, end_date, is_active
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
            is_active: subscription.is_active.unwrap_or_default(),
        })
    }

    async fn get_subscriptions_for_by_username(
        &self,
        username: String,
    ) -> Result<Vec<SubscriptionResponse>, String> {
        let user_service = UserService::new(self.pool.clone());

        let user: UserResponse = user_service.get_user(username).await?;

        let subscriptions = sqlx::query!(
            r#"
            SELECT id, user_id, plan_id, start_date, end_date, is_active
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
                is_active: subscription.is_active.unwrap_or_default(),
            })
            .collect())
    }
}
