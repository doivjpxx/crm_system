pub mod auth;
pub mod payments;
pub mod permissions;
pub mod plans;
pub mod resources;
pub mod roles;
pub mod subscriptions;
pub mod sys;
pub mod users;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    apps::app::AppState,
    apps::handlers::{health::health, sys::sys_login},
    apps::routes::{
        auth::auth_routes, payments::payment_routes, permissions::permission_routes,
        plans::plan_routes, resources::resource_routes, roles::role_routes,
        subscriptions::subscription_routes, sys::sys_routes, users::user_routes,
    },
};

pub struct AppRouter {
    pub app_state: Arc<AppState>,
}

impl AppRouter {
    pub fn new(app_state: Arc<AppState>) -> Self {
        tracing::info!("Creating AppRouter");
        Self { app_state }
    }

    pub fn create(&self) -> Router {
        let sys_pub_routes = Router::new().route("/login", post(sys_login));

        let api_routes = Router::new()
            .nest("/sys", sys_pub_routes)
            .nest("/sys", sys_routes())
            .nest("/auth", auth_routes())
            .nest("/roles", role_routes())
            .nest("/permissions", permission_routes())
            .nest("/users", user_routes())
            .nest("/plans", plan_routes())
            .nest("/resources", resource_routes())
            .nest("/subscriptions", subscription_routes())
            .nest("/payments", payment_routes());

        Router::new()
            .route("/health", get(health))
            .nest("/api", api_routes)
            .with_state(self.app_state.to_owned())
    }
}
