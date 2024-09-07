#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub refresh_secret: String,
    pub jwt_expire_in: usize,
}

impl Config {
    pub fn init() -> Self {
        let now = chrono::Utc::now();

        let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_owned());
        let port = std::env::var("PORT")
            .unwrap_or("3000".to_owned())
            .parse()
            .unwrap();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret: String = std::env::var("SECRET_KEY").expect("JWT_SECRET must be set");
        let refresh_secret: String =
            std::env::var("REFRESH_SECRET_KEY").expect("REFRESH_SECRET must be set");

        Self {
            host,
            port,
            database_url,
            jwt_secret,
            refresh_secret,
            jwt_expire_in: now.timestamp() as usize + 60 * 60,
        }
    }
}
