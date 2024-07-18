use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    app::AppState,
    handlers::{
        health::health,
        sys::{get_sys, sys_login},
        users::{create_user, get_current_user, get_user, login},
    },
    middlewares::{auth::auth_middleware, sys::sys_middleware},
};

pub struct AppRouter {
    pub app_state: Arc<AppState>,
}

impl AppRouter {
    pub fn new(self) -> Self {
        Self {
            app_state: self.app_state,
        }
    }

    pub fn create(&self) -> Router {
        let sys_pub_routes = Router::new().route("/login", post(sys_login));
        let sys_routes = Router::new()
            .route("/user", post(create_user))
            .route("/me", get(get_sys))
            .layer(axum::middleware::from_fn(sys_middleware));

        let user_routes = Router::new()
            .route("/profile/me", get(get_current_user))
            .route("/:username", get(get_user))
            .layer(axum::middleware::from_fn(auth_middleware));

        let auth_routes = Router::new().route("/login", post(login));

        let api_routes = Router::new()
            .nest("/sys", sys_pub_routes)
            .nest("/sys", sys_routes)
            .nest("/auth", auth_routes)
            .nest("/users", user_routes);

        Router::new()
            .route("/health", get(health))
            .nest("/api", api_routes)
            .with_state(self.app_state.to_owned())
    }
}
