use crate::AppState;
use axum::Router;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use microbin_frontend::components::guide::Guide;
use models::error_handling::AppError;

pub async fn guide(
    State(AppState { args, .. }): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let guide = dioxus_ssr::render_element(Guide(args.into()));
    Ok(Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(guide)?
        .into_response())
}

pub fn guide_router() -> Router<AppState> {
    Router::new().route("/guide", axum::routing::get(guide))
}
