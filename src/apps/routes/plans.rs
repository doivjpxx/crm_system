use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    apps::app::AppState,
    apps::handlers::plans::{get_plan, get_plans},
};

pub fn plan_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_plans))
        .route("/:id", get(get_plan))
}
