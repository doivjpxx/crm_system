use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    Json, RequestPartsExt,
};
use jsonwebtoken::{decode, encode, Header, TokenData, Validation};

use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::{Deserialize, Serialize};

use crate::infra::keys::{KEYS, REFRESH_KEYS};

use super::{
    sys_service::SysResponse,
    user_service::{ResourceForUser, SubscriptionForUser, UserWithSubscriptionResponse},
};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub id: uuid::Uuid,
    pub sub: String,
    pub username: String,
    pub iat: usize,
    pub exp: usize,
    pub is_sys: Option<bool>,
    pub subscription: Option<SubscriptionForUser>,
    pub resources: Vec<ResourceForUser>,
}

#[derive(Deserialize, Serialize)]
pub struct RefreshClaims {
    pub username: String,
    pub iat: usize,
    pub exp: usize,
}

impl Claims {
    pub fn encode_jwt(user: UserWithSubscriptionResponse) -> Result<String, (StatusCode, String)> {
        tracing::info!("claim_service --> encoding jwt for user");

        let now = chrono::Utc::now();
        let iat = now.timestamp() as usize;
        let exp = now.timestamp() as usize + 60 * 60;
        let claims = Claims {
            id: user.id,
            sub: user.email.clone(),
            username: user.username.clone(),
            iat,
            exp,
            is_sys: None,
            subscription: user.subscription,
            resources: user.resources,
        };

        encode(&Header::default(), &claims, &KEYS.encoding)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }

    pub fn encode_refresh_jwt(username: String) -> Result<String, (StatusCode, String)> {
        let now = chrono::Utc::now();
        let iat = now.timestamp() as usize;
        let exp = now.timestamp() as usize + 60 * 60 * 24 * 7;
        let claims = RefreshClaims { username, iat, exp };

        encode(&Header::default(), &claims, &REFRESH_KEYS.encoding)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }

    pub fn encode_jwt_sys(sys: SysResponse) -> Result<String, (StatusCode, String)> {
        tracing::info!("claim_service --> encoding jwt for sys");

        let now = chrono::Utc::now();
        let iat = now.timestamp() as usize;
        let exp = now.timestamp() as usize + 60 * 60;
        let claims = Claims {
            id: sys.id,
            sub: sys.username.clone(),
            username: sys.username.clone(),
            iat,
            exp,
            is_sys: Some(true),
            subscription: None,
            resources: vec![],
        };

        encode(&Header::default(), &claims, &KEYS.encoding).map_err(|e| {
            tracing::error!("claim_service --> Failed to encode jwt: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })
    }

    pub fn decode_jwt(jwt: &str) -> Result<TokenData<Claims>, (StatusCode, String)> {
        tracing::info!("claim_service --> decoding jwt");

        decode(jwt, &KEYS.decoding, &Validation::default()).map_err(|e| {
            tracing::error!("claim_service --> Failed to decode jwt: {:?}", e);
            (StatusCode::UNAUTHORIZED, e.to_string())
        })
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
