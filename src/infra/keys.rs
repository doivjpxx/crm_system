use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;

use super::configs::Config;

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
    let config: Config = Config::init();
    Keys::new(config.jwt_secret.as_bytes())
});

pub static REFRESH_KEYS: Lazy<Keys> = Lazy::new(|| {
    let config: Config = Config::init();
    Keys::new(config.refresh_secret.as_bytes())
});
