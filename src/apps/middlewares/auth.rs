use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse, Json};

use crate::infra::services::claim_service::Claims;

use super::get_jwt_decoded;

fn has_user_permission(req: &Request) -> Result<(), (StatusCode, String)> {
    let claims: Result<Claims, (StatusCode, String)> = get_jwt_decoded(req);

    match claims {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn auth_middleware(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match has_user_permission(&req) {
        Ok(_) => {
            let response = next.run(req).await;
            Ok(response)
        }
        Err(e) => {
            if e.0 == StatusCode::UNAUTHORIZED {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(
                        serde_json::json!({ "error": "Unauthorized", "message": e.1, "status": 401}),
                    ),
                ));
            }

            Err((
                StatusCode::FORBIDDEN,
                Json(serde_json::json!({ "error": "Forbidden", "message": e.1, "status": 403 })),
            ))
        }
    }
}
