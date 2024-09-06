use serde::Serialize;

#[derive(Serialize)]
pub struct PermissionResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}
