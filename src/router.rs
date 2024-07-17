use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    app::AppState,
    handlers::{health::health, users::create_user},
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/user", post(create_user))
        .with_state(app_state)
}
