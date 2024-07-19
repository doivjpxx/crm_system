use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreatePlanRequest {
    pub name: String,
    pub description: String,
    pub price: i64,
    pub is_active: bool,
    pub tags: Vec<String>,
    pub trial_days: Option<i32>,
}

#[derive(Serialize)]
pub struct PlanResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub price: i64,
    pub is_active: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub trial_days: Option<i32>,
    pub created_at: Option<NaiveDateTime>
}

pub struct PlanService {
    pub pool: sqlx::PgPool,
}

impl PlanService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_plan(&self, id: uuid::Uuid) -> Result<PlanResponse, String> {
        let plan = sqlx::query!(
            r#"
            SELECT id, name, description, price, is_active, tags, trial_days, created_at
            FROM plans
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get plan: {:?}", e);
            "Failed to get plan".to_string()
        })?;

        Ok(PlanResponse {
            id: plan.id,
            name: plan.name,
            description: plan.description,
            price: plan.price,
            is_active: plan.is_active,
            tags: plan.tags,
            trial_days: plan.trial_days,
            created_at: plan.created_at,
        })
    }

    pub async fn create_plan(&self, plan: CreatePlanRequest) -> Result<PlanResponse, String> {
        let plan = sqlx::query!(
            r#"
            INSERT INTO plans (name, description, price, is_active, tags, trial_days)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, description, price, is_active, tags, trial_days, created_at
            "#,
            plan.name,
            plan.description,
            plan.price,
            plan.is_active,
            plan.tags.as_slice(),
            plan.trial_days
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create plan: {:?}", e);
            "Failed to create plan".to_string()
        })?;

        Ok(PlanResponse {
            id: plan.id,
            name: plan.name,
            description: plan.description,
            price: plan.price,
            is_active: plan.is_active,
            tags: plan.tags,
            trial_days: plan.trial_days,
            created_at: plan.created_at,
        })
    }

    pub async fn get_plans(&self) -> Result<Vec<PlanResponse>, String> {
        let plans = sqlx::query!(
            r#"
            SELECT id, name, description, price, is_active, tags, trial_days, created_at
            FROM plans
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get plans: {:?}", e);
            "Failed to get plans".to_string()
        })?;

        Ok(plans
            .into_iter()
            .map(|plan| PlanResponse {
                id: plan.id,
                name: plan.name,
                description: plan.description,
                price: plan.price,
                is_active: plan.is_active,
                tags: plan.tags,
                trial_days: plan.trial_days,
                created_at: plan.created_at,
            })
            .collect())
    }

    pub async fn update_plan(&self, id: uuid::Uuid, plan: CreatePlanRequest) -> Result<PlanResponse, String> {
        let plan = sqlx::query!(
            r#"
            UPDATE plans
            SET name = $1, description = $2, price = $3, is_active = $4, tags = $5, trial_days = $6
            WHERE id = $7
            RETURNING id, name, description, price, is_active, tags, trial_days, created_at
            "#,
            plan.name,
            plan.description,
            plan.price,
            plan.is_active,
            plan.tags.as_slice(),
            plan.trial_days,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update plan: {:?}", e);
            "Failed to update plan".to_string()
        })?;

        Ok(PlanResponse {
            id: plan.id,
            name: plan.name,
            description: plan.description,
            price: plan.price,
            is_active: plan.is_active,
            tags: plan.tags,
            trial_days: plan.trial_days,
            created_at: plan.created_at,
        })
    }
}
