use std::sync::Arc;

use axum::{
    routing::{get, patch, post, put},
    Router,
};

use crate::{
    app::AppState,
    handlers::{
        health::health,
        payment::make_payment,
        permissions::get_permissions,
        plans::{create_plan, get_plan, get_plans, update_plan},
        resources::{
            create_resource, get_resource, get_resources, get_resources_by_plan, update_resource,
        },
        roles::{create_role, get_roles_by_user_created, update_role},
        subscriptions::{
            activate_subscription, create_subscription, get_subscription, get_subscription_by_user,
            get_subscriptions,
        },
        sys::{get_sys, sys_login},
        users::{
            change_password, create_user, get_current_user, get_user, get_users, login,
            refresh_token, register, update_user,
        },
    },
    middlewares::{auth::auth_middleware, create_role::allow_create_role, sys::sys_middleware},
};

pub struct AppRouter {
    pub app_state: Arc<AppState>,
}

impl AppRouter {
    pub fn new(app_state: Arc<AppState>) -> Self {
        tracing::info!("Creating AppRouter");
        Self { app_state }
    }

    pub fn create(&self) -> Router {
        let sys_pub_routes = Router::new().route("/login", post(sys_login));
        let sys_routes = Router::new()
            .route("/users", post(create_user).get(get_users))
            .route("/me", get(get_sys))
            .route("/plans", post(create_plan).put(update_plan))
            .route("/subscriptions", get(get_subscriptions))
            .route("/subscriptions/:id", patch(activate_subscription))
            .route("/resources", post(create_resource))
            .route("/resources/:id", put(update_resource))
            .layer(axum::middleware::from_fn(sys_middleware));

        let permission_routes = Router::new().route("/", get(get_permissions));

        let user_routes = Router::new()
            .route("/profile/me", get(get_current_user))
            .route("/change-password", post(change_password))
            .route("/:username", get(get_user).put(update_user))
            .layer(axum::middleware::from_fn(auth_middleware));

        let auth_routes = Router::new()
            .route("/login", post(login))
            .route("/register", post(register))
            .route(
                "/refresh-token",
                post(refresh_token).layer(axum::middleware::from_fn(auth_middleware)),
            );

        let role_routes = Router::new()
            .route(
                "/",
                post(create_role).layer(axum::middleware::from_fn(allow_create_role)),
            )
            .route("/user/:id", get(get_roles_by_user_created))
            .route("/:id", put(update_role))
            .layer(axum::middleware::from_fn(auth_middleware));
        let plan_routes = Router::new()
            .route("/", get(get_plans))
            .route("/:id", get(get_plan));

        let resource_routes = Router::new()
            .route("/", get(get_resources))
            .route("/:id", get(get_resource))
            .route("/plan/:plan_id", get(get_resources_by_plan));

        let subscription_routes = Router::new()
            .route("/", post(create_subscription))
            .route("/:id", get(get_subscription))
            .route("/user/:username", get(get_subscription_by_user))
            .layer(axum::middleware::from_fn(auth_middleware));

        let payment_routes = Router::new()
            .route("/", post(make_payment))
            .layer(axum::middleware::from_fn(auth_middleware));

        let api_routes = Router::new()
            .nest("/sys", sys_pub_routes)
            .nest("/sys", sys_routes)
            .nest("/auth", auth_routes)
            .nest("/roles", role_routes)
            .nest("/permissions", permission_routes)
            .nest("/users", user_routes)
            .nest("/plans", plan_routes)
            .nest("/resources", resource_routes)
            .nest("/subscriptions", subscription_routes)
            .nest("/payments", payment_routes);

        Router::new()
            .route("/health", get(health))
            .nest("/api", api_routes)
            .with_state(self.app_state.to_owned())
    }
}
