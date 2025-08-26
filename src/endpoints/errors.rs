use crate::AppState;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use microbin_frontend::components::error::Error;
use models::error_handling::AppError;

pub async fn not_found(
    State(AppState { args, .. }): State<AppState>,
) -> Result<Response, AppError> {
    let error = dioxus_ssr::render_element(Error(args.into()));

    let resp = Response::builder()
        .header("content-type", "text/html; charset=utf-8")
        .body(error)
        .map_err(AppError::from)?;
    Ok(resp.into_response())
}
