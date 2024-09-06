use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{app::AppState, handlers::permissions::get_permissions};

pub fn permission_routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_permissions))
}
