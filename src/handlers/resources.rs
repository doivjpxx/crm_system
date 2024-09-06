use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    app::AppState,
    dtos::resource_dtos::CreateResourceRequest,
    services::resource_service::{ResourceService, ResourceServiceImpl},
};

pub async fn create_resource(
    State(state): State<Arc<AppState>>,
    Json(resource): Json<CreateResourceRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service = ResourceService::new(state.pool.clone());

    match service.create_resource(resource).await {
        Ok(resource) => Ok((StatusCode::CREATED, Json(resource))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

pub async fn get_resources_by_plan(
    Path(plan_id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service = ResourceService::new(state.pool.clone());

    match service.get_resources_for_plan(plan_id).await {
        Ok(resources) => Ok((StatusCode::OK, Json(resources))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

pub async fn get_resources(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service = ResourceService::new(state.pool.clone());

    match service.get_resources().await {
        Ok(resources) => Ok((StatusCode::OK, Json(resources))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

pub async fn get_resource(
    Path(id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service = ResourceService::new(state.pool.clone());

    match service.get_resource(id).await {
        Ok(resource) => Ok((StatusCode::OK, Json(resource))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

pub async fn update_resource(
    Path(id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
    Json(resource): Json<CreateResourceRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service = ResourceService::new(state.pool.clone());

    match service.update_resource(id, resource).await {
        Ok(resource) => Ok((StatusCode::OK, Json(resource))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}
