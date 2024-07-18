pub mod auth;
pub mod sys;

use axum::{extract::Request, http::StatusCode};

use crate::services::claim_service::Claims;

fn get_jwt_decoded(req: &Request) -> Result<Claims, (StatusCode, String)> {
    let headers = req.headers();
    let auth_header = headers.get("Authorization");

    let auth_header = match auth_header {
        Some(auth_header) => auth_header,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "No Authorization header".to_owned(),
            ))
        }
    };

    let jwt_without_bearer = auth_header.to_str().unwrap().replace("Bearer ", "");
    let jwt_decoded = Claims::decode_jwt(&jwt_without_bearer);

    match jwt_decoded {
        Ok(jwt) => Ok(jwt.claims),
        Err(e) => Err((StatusCode::UNAUTHORIZED, e.1)),
    }
}
