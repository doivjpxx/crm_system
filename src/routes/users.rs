use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, patch, post},
    Router,
};

use crate::{
    app::AppState,
    handlers::users::{
        change_password, create_child_user, get_current_user, get_user, get_user_groups,
        get_user_groups_by_child_id, update_user,
    },
    middlewares::auth::auth_middleware,
};

pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/profile/me", get(get_current_user))
        .route("/change-password", post(change_password))
        .route("/:username/group", get(get_user_groups))
        .route(
            "/:username/group-by-child",
            get(get_user_groups_by_child_id),
        )
        .route("/:username/add", patch(create_child_user))
        .route("/:username", get(get_user).put(update_user))
        .layer(middleware::from_fn(auth_middleware))
}
