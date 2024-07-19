use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscription {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub plan_id: uuid::Uuid,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub is_active: bool,
    pub trial_start_date: chrono::DateTime<chrono::Utc>,
    pub trial_end_date: Option<chrono::DateTime<chrono::Utc>>,
}
