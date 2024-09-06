use chrono::NaiveDateTime;
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
    pub created_at: Option<NaiveDateTime>,
}

impl PlanResponse {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        description: String,
        price: i64,
        is_active: Option<bool>,
        tags: Option<Vec<String>>,
        trial_days: Option<i32>,
        created_at: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            price,
            is_active,
            tags,
            trial_days,
            created_at,
        }
    }
}
