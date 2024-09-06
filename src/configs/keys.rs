use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    Keys::new(secret.as_bytes())
});

pub static REFRESH_KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("REFRESH_SECRET_KEY").expect("REFRESH_SECRET_KEY must be set");
    Keys::new(secret.as_bytes())
});
