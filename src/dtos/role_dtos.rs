use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub description: String,
    pub created_by: uuid::Uuid,
}

#[derive(Serialize)]
pub struct RoleResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub created_by: uuid::Uuid,
    pub created_at: Option<chrono::NaiveDateTime>,
}
