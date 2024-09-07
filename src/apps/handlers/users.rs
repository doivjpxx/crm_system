use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    app::AppState,
    domain::dtos::user_dtos::{
        ChangePasswordRequest, CreateUserRequest, LoginRequest, UpdateUserRequest,
    },
    infra::services::{
        claim_service::Claims,
        user_group_service::{UserGroupService, UserGroupServiceImpl},
        user_service::{UserService, UserServiceImpl},
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

pub async fn get_users(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_service = UserService::new(state.pool.clone());

    match user_service.get_users().await {
        Ok(users) => Ok((StatusCode::OK, Json(users))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!(e)),
        )),
    }
}

pub async fn get_user(
    _: Claims,
    Path(username): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_service = UserService::new(state.pool.clone());

    match user_service.get_user(username).await {
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!(e)),
        )),
    }
}

pub async fn get_current_user(
    claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_service = UserService::new(state.pool.clone());

    match user_service.get_user(claims.username).await {
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

pub async fn register(
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

pub async fn update_user(
    _: Claims,
    Path(username): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(user): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_service = UserService::new(state.pool.clone());

    match user_service.update_user(username, user).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(serde_json::json!({ "message": "User updated" })),
        )),
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
        Ok(token) => Ok((
            StatusCode::OK,
            Json(
                serde_json::json!({ "token": token.0, "refresh_token": token.1, "type": "Bearer" }),
            ),
        )),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "Bad request", "message": e })),
        )),
    }
}

pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    refresh_token: String,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_service = UserService::new(state.pool.clone());

    match user_service.refresh_token(refresh_token).await {
        Ok(token) => Ok((
            StatusCode::OK,
            Json(serde_json::json!({ "token": token, "type": "Bearer" })),
        )),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "Bad request", "message": e })),
        )),
    }
}

pub async fn change_password(
    claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(user): Json<ChangePasswordRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_service = UserService::new(state.pool.clone());

    match user_service
        .change_password(claims.username, user.old_password, user.new_password)
        .await
    {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(serde_json::json!({ "message": "Password changed" })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

pub async fn create_child_user(
    _: Claims,
    Path(username): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(user): Json<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_service = UserService::new(state.pool.clone());

    match user_service.create_child_user(username, user).await {
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

pub async fn get_user_groups(
    _: Claims,
    Path(parent_id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_service = UserGroupService::new(state.pool.clone());

    match user_service.get_user_groups_by_parent_id(parent_id).await {
        Ok(user_groups) => Ok((StatusCode::OK, Json(user_groups))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!(e)),
        )),
    }
}

pub async fn get_user_groups_by_child_id(
    _: Claims,
    Path(child_id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_service = UserGroupService::new(state.pool.clone());

    match user_service.get_user_groups_by_child_id(child_id).await {
        Ok(user_groups) => Ok((StatusCode::OK, Json(user_groups))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!(e)),
        )),
    }
}
