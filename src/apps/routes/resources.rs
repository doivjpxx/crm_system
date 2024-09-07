use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    apps::app::AppState,
    apps::handlers::resources::{get_resource, get_resources, get_resources_by_plan},
};

pub fn resource_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_resources))
        .route("/:id", get(get_resource))
        .route("/plan/:plan_id", get(get_resources_by_plan))
}
