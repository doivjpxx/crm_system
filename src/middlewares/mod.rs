pub mod sys;
pub mod auth;

use axum::{extract::Request, http::StatusCode};

use crate::services::claim_service::Claims;

fn get_jwt_decoded(req: &Request) -> Result<Claims, (StatusCode, String)> {
    let headers = req.headers();
    let auth_header = headers.get("Authorization").unwrap();
    let jwt_without_bearer = auth_header.to_str().unwrap().replace("Bearer ", "");
    let jwt_decoded = Claims::decode_jwt(&jwt_without_bearer);

    match jwt_decoded {
        Ok(jwt) => Ok(jwt.claims),
        Err(e) => Err((StatusCode::UNAUTHORIZED, e.1)),
    }
}
