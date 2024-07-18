use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    app::AppState,
    services::sys_service::{SysLoginRequest, SysService},
};

pub async fn sys_login(
    State(state): State<Arc<AppState>>,
    Json(sys_user): Json<SysLoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let sys_service = SysService::new(state.pool.clone());

    match sys_service.login(sys_user).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(serde_json::json!({ "message": "Logged in" })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}
