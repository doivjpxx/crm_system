use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    app::AppState,
    services::{
        claim_service::Claims,
        user_service::{CreateUserRequest, LoginRequest, UserService},
    },
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

pub async fn get_user(
    claims: Claims,
    Path(username): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_service = UserService::new(state.pool.clone());

    match user_service.get_user(username).await {
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(user): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_service = UserService::new(state.pool.clone());

    match user_service.login(user.username, user.password).await {
        Ok(token) => Ok((StatusCode::OK, Json(serde_json::json!({ "token": token })))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}
