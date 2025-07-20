use crate::args::{Args, ARGS};
use crate::AppState;
use askama::Template;
use axum::response::{IntoResponse, Response};
use axum::Router;

#[derive(Template)]
#[template(path = "guide.html")]
struct Guide<'a> {
    args: &'a Args,
}

pub async fn guide() -> impl IntoResponse {
    Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(Guide { args: &ARGS }.render().unwrap())
        .unwrap()
}

pub fn guide_router() -> Router<AppState> {
    Router::new().route("/guide", axum::routing::get(guide))
}
