use serde::{Deserialize, Serialize};

use super::{plan_dtos::PlanResponse, subscription_dtos::SubscriptionResponse};

#[derive(Debug, Deserialize)]
pub struct CreatePaymentRequest {
    pub subscription_id: uuid::Uuid,
    pub amount: i64,
    pub payment_method: String,
}

#[derive(Serialize)]
pub struct PaymentResponse {
    pub id: uuid::Uuid,
    pub subscription_id: Option<uuid::Uuid>,
    pub amount: i64,
    pub payment_date: Option<chrono::NaiveDateTime>,
    pub payment_method: String,
}

#[derive(Serialize)]
pub struct PaymentForSysResponse {
    pub id: uuid::Uuid,
    pub amount: i64,
    pub payment_date: Option<chrono::NaiveDateTime>,
    pub payment_method: String,
    pub subscription: SubscriptionResponse,
    pub plan: PlanResponse,
    pub username: String,
    pub email: String,
    pub user_id: uuid::Uuid,
}
