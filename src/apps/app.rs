use std::{env, sync::Arc};

use axum::{
    extract::{MatchedPath, Request},
    http::{
        header::{self},
        Method,
    },
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info_span;

use crate::{apps::routes::AppRouter, infra::configs::Config};

pub struct AppState {
    pub pool: sqlx::PgPool,
}

pub async fn run_app(app_state: Arc<AppState>) {
    let config: Config = Config::init();
    let addr = format!("{}:{}", config.host, config.port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    let cors = CorsLayer::new()
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_origin(Any)
        .allow_headers(vec![
            header::ACCEPT,
            header::ACCEPT_LANGUAGE,
            header::AUTHORIZATION,
            header::CONTENT_LANGUAGE,
            header::CONTENT_TYPE,
        ]);

    tracing::info!("CORS enabled");

    let router = AppRouter::new(app_state).create().layer(cors).layer(
        TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);

            info_span!(
                "http_request",
                method = ?request.method(),
                matched_path,
                some_other_field = tracing::field::Empty,
            )
        }),
    );

    tracing::info!("Server started successfully at {}", addr);

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
