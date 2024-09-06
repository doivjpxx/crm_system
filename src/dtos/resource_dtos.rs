use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateResourceRequest {
    pub plan_id: uuid::Uuid,
    pub max: i64,
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct ResourceResponse {
    pub id: uuid::Uuid,
    pub plan_id: uuid::Uuid,
    pub max: i64,
    pub name: String,
    pub description: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}
