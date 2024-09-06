use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    app::AppState,
    dtos::plan_dtos::CreatePlanRequest,
    services::plan_service::{PlanService, PlanServiceImpl},
};

pub async fn get_plan(
    Path(id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service = PlanService::new(state.pool.clone());

    match service.get_plan(id).await {
        Ok(pl) => Ok((StatusCode::OK, Json(pl))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

pub async fn create_plan(
    State(state): State<Arc<AppState>>,
    Json(plan): Json<CreatePlanRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service = PlanService::new(state.pool.clone());

    match service.create_plan(plan).await {
        Ok(pl) => Ok((
            StatusCode::CREATED,
            Json(serde_json::json!({ "message": "Plan created", "data": pl })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

pub async fn get_plans(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service = PlanService::new(state.pool.clone());

    match service.get_plans().await {
        Ok(plans) => Ok((StatusCode::OK, Json(plans))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

pub async fn update_plan(
    Path(id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
    Json(plan): Json<CreatePlanRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let service = PlanService::new(state.pool.clone());

    match service.update_plan(id, plan).await {
        Ok(pl) => Ok((
            StatusCode::OK,
            Json(serde_json::json!({ "message": "Plan updated", "data": pl })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}
