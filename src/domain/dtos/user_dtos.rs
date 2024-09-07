use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub name: Option<String>,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub username: String,
    pub email: String,
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub name: String,
    pub email: String,
    pub is_sys: Option<bool>,
    pub created_at: chrono::NaiveDateTime,
}

impl UserResponse {
    pub fn new(id: uuid::Uuid, username: String, name: String, email: String) -> Self {
        Self {
            id,
            username,
            name,
            email,
            is_sys: None,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}
