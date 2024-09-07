use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    app::AppState,
    domain::dtos::subscription_dtos::CreateSubscriptionRequest,
    infra::services::{
        claim_service::Claims,
        subscription_service::{SubscriptionService, SubscriptionServiceImpl},
    },
};

pub async fn create_subscription(
    State(state): State<Arc<AppState>>,
    Json(subscription): Json<CreateSubscriptionRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let sub_service = SubscriptionService::new(state.pool.clone());

    match sub_service.create_subscription(subscription).await {
        Ok(sub) => Ok((
            StatusCode::CREATED,
            Json(serde_json::json!({ "message": "Subscription created", "data": sub })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

pub async fn activate_subscription(
    Path(id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let sub_service = SubscriptionService::new(state.pool.clone());

    match sub_service.activate_subscription(id).await {
        Ok(sub) => Ok((
            StatusCode::CREATED,
            Json(serde_json::json!({ "message": "Subscription activated", "data": sub })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

pub async fn deactivate_subscription(
    Path(id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let sub_service = SubscriptionService::new(state.pool.clone());

    match sub_service.deactivate_subscription(id).await {
        Ok(sub) => Ok((
            StatusCode::CREATED,
            Json(serde_json::json!({ "message": "Subscription deactivated", "data": sub })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

pub async fn get_subscriptions(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let sub_service = SubscriptionService::new(state.pool.clone());

    match sub_service.get_subscriptions().await {
        Ok(subs) => Ok((StatusCode::OK, Json(subs))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!(e)),
        )),
    }
}

pub async fn get_subscription(
    Path(id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let sub_service = SubscriptionService::new(state.pool.clone());

    match sub_service.get_subscription(id).await {
        Ok(sub) => Ok((StatusCode::OK, Json(serde_json::json!(sub)))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

pub async fn get_subscription_by_user(
    claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let username = claims.username;

    let sub_service = SubscriptionService::new(state.pool.clone());

    match sub_service
        .get_subscriptions_for_by_username(username)
        .await
    {
        Ok(sub) => Ok((StatusCode::OK, Json(serde_json::json!(sub)))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}
