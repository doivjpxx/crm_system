use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    app::AppState,
    services::payment_service::{CreatePaymentRequest, PaymentService},
};

pub async fn make_payment(
    State(state): State<Arc<AppState>>,
    Json(payment): Json<CreatePaymentRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let payment_service = PaymentService::new(state.pool.clone());

    match payment_service.make_payment(payment).await {
        Ok(payment) => Ok((
            StatusCode::CREATED,
            Json(serde_json::json!({ "message": "Payment created", "data": payment })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}
