use std::sync::Arc;

use apps::app::{self, AppState};
use dotenv::dotenv;
use infra::{db::postgres, tracing::init_tracing};

mod apps;
mod domain;
mod infra;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_tracing();

    let pool: sqlx::Pool<sqlx::Postgres> = postgres::connect().await;

    app::run_app(Arc::new(AppState { pool })).await;
}
