use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    app::AppState,
    handlers::{
        health::health,
        sys::sys_login,
        users::{create_user, get_user, login},
    },
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/sys/login", post(sys_login))
        .route("/user", post(create_user))
        .route("/login", post(login))
        .route("/user/:id", get(get_user))
        .with_state(app_state)
}
