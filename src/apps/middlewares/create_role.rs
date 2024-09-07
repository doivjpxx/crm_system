use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse, Json};

use super::get_jwt_decoded;

pub async fn allow_create_role(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let claims = get_jwt_decoded(&req);

    match claims {
        Ok(claims) => {
            if claims.is_sys.unwrap() {
                let subscription = claims.subscription;

                if subscription.is_none() {
                    return Err((
                        StatusCode::FORBIDDEN,
                        Json(serde_json::json!({ "error": "Forbidden" })),
                    ));
                }

                let now = chrono::Utc::now();
                let subscription_end_date = subscription.clone().unwrap().end_date.unwrap();
                let subscription_trial_end_date =
                    subscription.clone().unwrap().trial_end_date.unwrap();

                if now.timestamp() > subscription_end_date.and_utc().timestamp()
                    || now.timestamp() > subscription_trial_end_date.and_utc().timestamp()
                {
                    return Err((
                        StatusCode::FORBIDDEN,
                        Json(serde_json::json!({ "error": "Forbidden" })),
                    ));
                }

                let response = next.run(req).await;
                Ok(response)
            } else {
                Err((
                    StatusCode::FORBIDDEN,
                    Json(serde_json::json!({ "error": "Forbidden" })),
                ))
            }
        }
        Err(e) => Err((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({ "error": "Forbidden", "message": e.1 })),
        )),
    }
}
