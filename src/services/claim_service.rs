use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    Json, RequestPartsExt,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use once_cell::sync::Lazy;

use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::{Deserialize, Serialize};

use super::{sys_service::SysResponse, user_service::UserResponse};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub iat: usize,
    pub exp: usize,
    pub is_sys: Option<bool>,
}

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

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    Keys::new(secret.as_bytes())
});

impl Claims {
    pub fn encode_jwt(user: UserResponse) -> Result<String, (StatusCode, String)> {
        let now = chrono::Utc::now();
        let iat = now.timestamp() as usize;
        let exp = now.timestamp() as usize + 60 * 60;
        let claims = Claims {
            sub: user.email.clone(),
            username: user.username.clone(),
            iat,
            exp,
            is_sys: None,
        };

        return encode(&Header::default(), &claims, &KEYS.encoding)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    }

    pub fn encode_jwt_sys(sys: SysResponse) -> Result<String, (StatusCode, String)> {
        let now = chrono::Utc::now();
        let iat = now.timestamp() as usize;
        let exp = now.timestamp() as usize + 60 * 60;
        let claims = Claims {
            sub: sys.username.clone(),
            username: sys.username.clone(),
            iat,
            exp,
            is_sys: Some(true),
        };

        return encode(&Header::default(), &claims, &KEYS.encoding)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    }

    pub fn decode_jwt(jwt: &str) -> Result<TokenData<Claims>, (StatusCode, String)> {
        decode(jwt, &KEYS.decoding, &Validation::default())
            .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(serde_json::json!({
                        "err": "Unauthorized"
                    })),
                )
            })?;
        // Decode the user data
        let token_data = Claims::decode_jwt(bearer.token()).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "err": "Unauthorized"
                })),
            )
        })?;

        Ok(token_data.claims)
    }
}
