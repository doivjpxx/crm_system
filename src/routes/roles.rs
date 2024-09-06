use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};

use crate::{
    app::AppState,
    handlers::roles::{create_role, get_roles_by_user_created, update_role},
    middlewares::{auth::auth_middleware, create_role::allow_create_role},
};

pub fn role_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            post(create_role).layer(axum::middleware::from_fn(allow_create_role)),
        )
        .route("/user/:id", get(get_roles_by_user_created))
        .route("/:id", put(update_role))
        .layer(middleware::from_fn(auth_middleware))
}
