use crate::args::{ARGS, Args};
use crate::error_handling::AppError;
use askama::Template;
use axum::response::{IntoResponse, Response};

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate<'a> {
    pub args: &'a Args,
}

pub async fn not_found() -> Result<Response, AppError> {
    let resp = Response::builder()
        .header("content-type", "text/html; charset=utf-8")
        .body(ErrorTemplate { args: &ARGS }.render()?)
        .map_err(AppError::from)?;
    Ok(resp.into_response())
}
