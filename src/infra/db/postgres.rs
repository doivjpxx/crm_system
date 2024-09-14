use sqlx::PgPool;

use crate::infra::configs::Config;

pub async fn connect() -> PgPool {
    let config: Config = Config::init();
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to create pool");

    tracing::info!(
        "Connected to database with address: {}",
        config.database_url
    );

    pool
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connect() {
        dotenv().ok();
        let pool: sqlx::Pool<sqlx::Postgres> = connect().await;
        assert_eq!(pool.is_closed(), false);
    }
}
