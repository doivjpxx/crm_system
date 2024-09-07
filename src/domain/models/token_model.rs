use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenModel {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub token: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
