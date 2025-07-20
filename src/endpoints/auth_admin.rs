use crate::args::{Args, ARGS};
use crate::AppState;
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

async fn auth_admin() -> impl IntoResponse {
    Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(
            AuthAdmin {
                args: &ARGS,
                status: "".to_string(),
            }
            .render()
            .unwrap(),
        )
        .unwrap()
}

async fn auth_admin_with_status(Path(status): Path<String>) -> impl IntoResponse {
    Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(
            AuthAdmin {
                args: &ARGS,
                status: status.to_string(),
            }
            .render()
            .unwrap(),
        )
        .unwrap()
}

pub fn auth_admin_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/auth_admin", axum::routing::get(auth_admin))
        .route(
            "/auth_admin/{status}",
            axum::routing::get(auth_admin_with_status),
        )
}
