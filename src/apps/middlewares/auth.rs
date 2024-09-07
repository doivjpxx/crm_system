use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse, Json};

use super::get_jwt_decoded;

fn has_user_permission(req: &Request) -> Result<(), String> {
    let claims = get_jwt_decoded(req);

    match claims {
        Ok(_) => Ok(()),
        Err(e) => Err(e.1),
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
        Err(e) => Err((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({ "error": "Forbidden", "message": e })),
        )),
    }
}
