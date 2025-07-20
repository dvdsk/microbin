use crate::args::{Args, ARGS};
use askama::Template;
use axum::response::{IntoResponse, Response};

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate<'a> {
    pub args: &'a Args,
}

pub async fn not_found() -> Response {
    let resp = Response::builder()
        .header("content-type", "text/html; charset=utf-8")
        .body(ErrorTemplate { args: &ARGS }.render().unwrap())
        .unwrap();
    resp.into_response()
}
