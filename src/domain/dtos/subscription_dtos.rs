use serde::{Deserialize, Serialize};

use super::{plan_dtos::PlanResponse, user_dtos::UserResponse};

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
    pub is_active: bool,
}

#[derive(Serialize)]
pub struct SubscriptionForSysResponse {
    pub id: uuid::Uuid,
    pub user_id: Option<uuid::Uuid>,
    pub plan_id: Option<uuid::Uuid>,
    pub start_date: Option<chrono::NaiveDateTime>,
    pub end_date: Option<chrono::NaiveDateTime>,
    pub user: UserResponse,
    pub plan: PlanResponse,
}
