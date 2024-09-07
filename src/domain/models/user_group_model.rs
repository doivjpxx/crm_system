use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGroupModel {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub parent_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
