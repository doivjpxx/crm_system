use std::sync::Arc;

use axum::{middleware, routing::post, Router};

use crate::{
    apps::app::AppState,
    apps::handlers::users::{login, refresh_token, register},
    apps::middlewares::auth::auth_middleware,
};

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route(
            "/refresh-token",
            post(refresh_token).layer(middleware::from_fn(auth_middleware)),
        )
}
