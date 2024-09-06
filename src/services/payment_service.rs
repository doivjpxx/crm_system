use crate::dtos::{
    payment_dtos::{CreatePaymentRequest, PaymentForSysResponse, PaymentResponse},
    plan_dtos::PlanResponse,
    subscription_dtos::SubscriptionResponse,
};

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

    pub async fn get_payments(&self) -> Result<Vec<PaymentForSysResponse>, String> {
        let payments = sqlx::query!(
            r#"
            SELECT p.id, p.subscription_id, p.amount, p.payment_date, p.payment_method, s.user_id, s.plan_id, s.start_date, s.end_date, s.is_active, pl.name, pl.price, pl.description, pl.trial_days, u.username, u.name as user_name, u.email
            FROM payments as p
            INNER JOIN subscriptions as s ON p.subscription_id = s.id
            INNER JOIN plans as pl ON s.plan_id = pl.id
            INNER JOIN users as u ON s.user_id = u.id
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get payments: {:?}", e);
            "Failed to get payments".to_string()
        })?;

        Ok(payments
            .into_iter()
            .map(|payment| PaymentForSysResponse {
                id: payment.id,
                amount: payment.amount,
                payment_date: payment.payment_date,
                payment_method: payment.payment_method.unwrap_or_default(),
                user_id: payment.user_id.unwrap_or_default(),
                username: payment.username,
                email: payment.email,
                subscription: SubscriptionResponse {
                    id: payment.subscription_id.unwrap(),
                    user_id: payment.user_id,
                    plan_id: payment.plan_id,
                    start_date: payment.start_date,
                    end_date: payment.end_date,
                    is_active: payment.is_active.unwrap_or_default(),
                },
                plan: PlanResponse {
                    id: payment.plan_id.unwrap(),
                    name: payment.name,
                    price: payment.price,
                    trial_days: None,
                    description: payment.description,
                    created_at: None,
                    tags: None,
                    is_active: None,
                },
            })
            .collect())
    }
}
