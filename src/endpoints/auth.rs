use crate::AppState;
use axum::extract::{Multipart, Request, State};
use axum::middleware::Next;
use axum::response::Response;
use base64::Engine;
use base64::engine::general_purpose;
use models::error_handling::AppError;
use reqwest::StatusCode;

pub async fn auth_validator(
    State(AppState { args, .. }): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
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
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("WWW-Authenticate", "Basic realm=\"Restricted Area\"")
            .body("Unauthorized".into())?);
    }

    if let Some(username) = username
        && username != args.auth_admin_username
    {
        return Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body("Forbidden".into())?);
    }

    if let Some(password) = password
        && password != args.auth_admin_password
    {
        return Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body("Forbidden".into())?);
    }

    Ok(next.run(req).await)
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
