use crate::args::{Args};
use crate::error_handling::AppError;
use askama::Template;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use crate::AppState;

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate<'a> {
    pub args: &'a Args,
}

pub async fn not_found(State(data): State<AppState>) -> Result<Response, AppError> {
    let resp = Response::builder()
        .header("content-type", "text/html; charset=utf-8")
        .body(ErrorTemplate { args: &data.args }.render()?)
        .map_err(AppError::from)?;
    Ok(resp.into_response())
}
