use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    app::AppState,
    services::permission_service::{PermissionService, PermissionServiceImpl},
};

pub async fn get_permissions(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service = PermissionService::new(state.pool.clone());

    match service.get_permissions().await {
        Ok(permissions) => Ok((StatusCode::OK, Json(permissions))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}
