use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    app::AppState,
    dtos::role_dtos::CreateRoleRequest,
    services::role_service::{RoleService, RoleServiceImpl},
};

pub async fn create_role(
    State(state): State<Arc<AppState>>,
    Json(role): Json<CreateRoleRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service = RoleService::new(state.pool.clone());

    match service.create_role(role).await {
        Ok(role) => Ok((StatusCode::CREATED, Json(role))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

pub async fn update_role(
    State(state): State<Arc<AppState>>,
    Json(role): Json<CreateRoleRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service = RoleService::new(state.pool.clone());

    match service.update_role(role).await {
        Ok(role) => Ok((StatusCode::OK, Json(role))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

pub async fn get_roles_by_user_created(
    Path(id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service = RoleService::new(state.pool.clone());

    match service.get_roles_by_user_created(id).await {
        Ok(roles) => Ok((StatusCode::OK, Json(roles))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}
