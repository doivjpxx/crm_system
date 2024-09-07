use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{apps::app::AppState, apps::handlers::permissions::get_permissions};

pub fn permission_routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_permissions))
}
