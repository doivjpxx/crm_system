use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, patch, post, put},
    Router,
};

use crate::{
    apps::app::AppState,
    apps::handlers::{
        payment::get_payments_for_sys,
        plans::{create_plan, update_plan},
        resources::{create_resource, update_resource},
        subscriptions::{activate_subscription, deactivate_subscription, get_subscriptions},
        sys::get_sys,
        users::{create_user, get_users},
    },
    apps::middlewares::sys::sys_middleware,
};

pub fn sys_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", post(create_user).get(get_users))
        .route("/me", get(get_sys))
        .route("/plans", post(create_plan).put(update_plan))
        .route("/payments", get(get_payments_for_sys))
        .route("/subscriptions", get(get_subscriptions))
        .route("/subscriptions/:id", patch(activate_subscription))
        .route(
            "/subscriptions/:id/deactivate",
            patch(deactivate_subscription),
        )
        .route("/resources", post(create_resource))
        .route("/resources/:id", put(update_resource))
        .layer(middleware::from_fn(sys_middleware))
}
