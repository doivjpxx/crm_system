use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{app::AppState, handlers::health::health};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health))
        .with_state(app_state)
}
