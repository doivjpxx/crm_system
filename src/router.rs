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
    let sys_routes = Router::new()
        .route("/login", post(sys_login))
        .route("/user/create", post(create_user));

    let user_routes = Router::new().route("/:username", get(get_user));

    let auth_routes = Router::new().route("/login", post(login));

    let api_routes = Router::new()
        .nest("/sys", sys_routes)
        .nest("/auth", auth_routes)
        .nest("/users", user_routes);

    Router::new()
        .route("/health", get(health))
        .nest("/api", api_routes)
        .with_state(app_state)
}
