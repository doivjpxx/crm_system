use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionModel {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
