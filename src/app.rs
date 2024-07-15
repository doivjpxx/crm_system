use std::{env, sync::Arc};

use crate::router;

pub struct AppState {
    pub pool: sqlx::PgPool,
}

pub async fn run_app(app_state: Arc<AppState>) {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    let router = router::create_router(app_state);

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
