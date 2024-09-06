use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    app::AppState,
    handlers::subscriptions::{create_subscription, get_subscription, get_subscription_by_user},
    middlewares::auth::auth_middleware,
};

pub fn subscription_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_subscription))
        .route("/:id", get(get_subscription))
        .route("/user/:username", get(get_subscription_by_user))
        .layer(middleware::from_fn(auth_middleware))
}
