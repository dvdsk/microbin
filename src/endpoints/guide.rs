use crate::AppState;
use crate::args::{Args};
use crate::error_handling::AppError;
use askama::Template;
use axum::extract::State;
use axum::Router;
use axum::response::{IntoResponse, Response};

#[derive(Template)]
#[template(path = "guide.html")]
struct Guide<'a> {
    args: &'a Args,
}

pub async fn guide(State(data): State<AppState>) -> Result<impl IntoResponse, AppError> {
    Ok(Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(Guide { args: &data.args }.render()?)?)
}

pub fn guide_router() -> Router<AppState> {
    Router::new().route("/guide", axum::routing::get(guide))
}
