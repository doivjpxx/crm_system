use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Payment {
    pub id: uuid::Uuid,
    pub subscription_id: uuid::Uuid,
    pub amount: f64,
    pub payment_date: chrono::DateTime<chrono::Utc>,
    pub payment_method: String,
}
