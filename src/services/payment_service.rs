use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PaymentResponse {
    pub id: uuid::Uuid,
    pub subscription_id: Option<uuid::Uuid>,
    pub amount: i64,
    pub payment_date: Option<chrono::NaiveDateTime>,
    pub payment_method: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePaymentRequest {
    pub subscription_id: uuid::Uuid,
    pub amount: i64,
    pub payment_method: String,
}

pub struct PaymentService {
    pub pool: sqlx::PgPool,
}

impl PaymentService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn make_payment(
        &self,
        payment: CreatePaymentRequest,
    ) -> Result<PaymentResponse, String> {
        let payment = sqlx::query!(
            r#"
            INSERT INTO payments (subscription_id, amount, payment_method)
            VALUES ($1, $2, $3)
            RETURNING id, subscription_id, amount, payment_method, payment_date
            "#,
            payment.subscription_id,
            payment.amount,
            payment.payment_method
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to make payment: {:?}", e);
            "Failed to make payment".to_string()
        })
        .unwrap();

        Ok(PaymentResponse {
            id: payment.id,
            subscription_id: payment.subscription_id,
            amount: payment.amount,
            payment_date: payment.payment_date,
            payment_method: payment.payment_method.unwrap_or_default(),
        })
    }
}
