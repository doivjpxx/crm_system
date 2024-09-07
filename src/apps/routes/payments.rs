use std::sync::Arc;

use axum::{middleware, routing::post, Router};

use crate::{apps::app::AppState, apps::handlers::payment::make_payment, apps::middlewares::auth::auth_middleware};

pub fn payment_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(make_payment))
        .layer(middleware::from_fn(auth_middleware))
}
