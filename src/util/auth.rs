use crate::args::ARGS;
use crate::error_handling::AppError;
use axum::extract::{Multipart, Request};
use axum::middleware::Next;
use axum::response::Response;
use base64::engine::general_purpose;
use base64::Engine;

pub async fn auth_validator(req: Request, next: Next) -> Response {
    let (username, password) = req
        .headers()
        .get("authorization")
        .and_then(|auth| auth.to_str().ok())
        .and_then(|auth_str| {
            let parts: Vec<&str> = auth_str.splitn(2, ' ').collect();
            if parts.len() == 2 && parts[0] == "Basic" {
                let decoded = general_purpose::STANDARD.decode(parts[1]).ok()?;
                let credentials = String::from_utf8(decoded).ok()?;
                let creds: Vec<&str> = credentials.splitn(2, ':').collect();
                if creds.len() == 2 {
                    Some((creds[0].to_string(), creds[1].to_string()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .map(|(user, password)| (Some(user), Some(password)))
        .unwrap_or((None, None));
    if username.is_none() || password.is_none() {
        return Response::builder()
            .status(401)
            .header("WWW-Authenticate", "Basic realm=\"Restricted Area\"")
            .body("Unauthorized".into())
            .unwrap();
    }
    if username.unwrap() != ARGS.auth_admin_username
        || password.unwrap() != ARGS.auth_admin_password
    {
        Response::builder()
            .status(403)
            .body("Forbidden".into())
            .unwrap()
    } else {
        next.run(req).await
    }
}

pub async fn password_from_multipart(mut payload: Multipart) -> Result<String, AppError> {
    let mut password = String::new();

    while let Some(field) = payload.next_field().await? {
        if field.name() == Some("password") {
            let password_bytes = field.bytes().await.unwrap_or(bytes::Bytes::new());
            password = String::from_utf8_lossy(&password_bytes).to_string();
        }
    }
    Ok(password)
}
