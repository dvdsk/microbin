use crate::AppState;
use crate::args::{ARGS, Args};
use crate::error_handling::AppError;
use askama::Template;
use axum::extract::Path;
use axum::http::Response;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "auth_admin.html")]
struct AuthAdmin<'a> {
    args: &'a Args,
    status: String,
}

async fn auth_admin() -> Result<impl IntoResponse, AppError> {
    Ok(Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(
            AuthAdmin {
                args: &ARGS,
                status: "".to_string(),
            }
            .render()?,
        )?)
}

async fn auth_admin_with_status(Path(status): Path<String>) -> Result<impl IntoResponse, AppError> {
    Ok(Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(
            AuthAdmin {
                args: &ARGS,
                status: status.to_string(),
            }
            .render()?,
        )?)
}

pub fn auth_admin_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/auth_admin", axum::routing::get(auth_admin))
        .route(
            "/auth_admin/{status}",
            axum::routing::get(auth_admin_with_status),
        )
}
