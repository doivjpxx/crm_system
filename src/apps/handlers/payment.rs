use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    app::AppState, domain::dtos::payment_dtos::CreatePaymentRequest,
    infra::services::payment_service::PaymentService,
};

pub async fn make_payment(
    State(state): State<Arc<AppState>>,
    Json(payment): Json<CreatePaymentRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let payment_service = PaymentService::new(state.pool.clone());

    match payment_service.make_payment(payment).await {
        Ok(payment) => Ok((StatusCode::CREATED, Json(payment))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

pub async fn get_payments_for_sys(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let payment_service = PaymentService::new(state.pool.clone());

    match payment_service.get_payments().await {
        Ok(payments) => Ok((StatusCode::OK, Json(payments))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}
