use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

pub async fn connect() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    tracing::info!("Connected to database with address: {}", database_url);

    pool
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connect() {
        let pool: sqlx::Pool<sqlx::Postgres> = connect().await;
        assert_eq!(pool.is_closed(), false);
    }
}
