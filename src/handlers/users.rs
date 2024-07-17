use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    app::AppState,
    services::user_service::{CreateUserRequest, UserService},
};

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(user): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_service = UserService::new(state.pool.clone());

    match user_service.create_user(user).await {
        Ok(_) => Ok((
            StatusCode::CREATED,
            Json(serde_json::json!({ "message": "User created" })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}
