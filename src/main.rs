use std::sync::Arc;

use app::AppState;
use dotenv::dotenv;

mod app;
mod configs;
mod db;
mod dtos;
mod handlers;
mod middlewares;
mod models;
mod router;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = db::connect().await;

    app::run_app(Arc::new(AppState { pool })).await;
}
