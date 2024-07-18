use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlanModel {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub is_active: bool,
    pub tags: Vec<String>,
    pub trial_days: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
