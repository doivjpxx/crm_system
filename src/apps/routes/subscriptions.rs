use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    apps::app::AppState,
    apps::handlers::subscriptions::{create_subscription, get_subscription, get_subscription_by_user},
    apps::middlewares::auth::auth_middleware,
};

pub fn subscription_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_subscription))
        .route("/:id", get(get_subscription))
        .route("/user/:username", get(get_subscription_by_user))
        .layer(middleware::from_fn(auth_middleware))
}
