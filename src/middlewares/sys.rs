use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse};

use super::get_jwt_decoded;

fn has_sys_permission(req: &Request) -> Result<(), ()> {
    let claims = get_jwt_decoded(req);

    if claims.is_ok() {
        let claims = claims.unwrap();
        if claims.is_sys.is_some() {
            Ok(())
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

pub async fn sys_middleware(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match has_sys_permission(&req) {
        Ok(_) => {
            let response = next.run(req).await;
            Ok(response)
        }
        Err(_) => Err((StatusCode::FORBIDDEN, "forbidden".to_owned())),
    }
}
