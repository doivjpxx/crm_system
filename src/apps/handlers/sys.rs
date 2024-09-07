use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    app::AppState,
    infra::services::{
        claim_service::Claims,
        sys_service::{SysLoginRequest, SysService},
    },
};

pub async fn sys_login(
    State(state): State<Arc<AppState>>,
    Json(sys_user): Json<SysLoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let sys_service = SysService::new(state.pool.clone());

    match sys_service.login(sys_user).await {
        Ok(token) => Ok((
            StatusCode::OK,
            Json(serde_json::json!({ "token": token, "type": "Bearer" })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

pub async fn get_sys(
    claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let sys_service = SysService::new(state.pool.clone());

    match sys_service.get_sys(claims.username).await {
        Ok(sys) => Ok((StatusCode::OK, Json(serde_json::json!(sys)))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}
