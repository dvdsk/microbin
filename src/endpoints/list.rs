use crate::args::{Args, ARGS};
use crate::pasta::Pasta;
use crate::util::misc::remove_expired;
use crate::AppState;
use askama::Template;
use axum::extract::State;
use axum::response::IntoResponse;
use reqwest::{header, StatusCode};

#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate<'a> {
    pastas: &'a Vec<Pasta>,
    args: &'a Args,
}

pub async fn list(State(data): State<AppState>) -> impl IntoResponse {
    if ARGS.no_listing {
        return (
            StatusCode::FOUND,
            [(header::LOCATION, format!("{}/", ARGS.public_path_as_str()))],
            "".to_string(),
        );
    }

    let mut pastas = data.pastas.lock().unwrap();

    remove_expired(&mut pastas);

    // sort pastas in reverse-chronological order of creation time
    pastas.sort_by(|a, b| b.created.cmp(&a.created));

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
        ListTemplate {
            pastas: &pastas,
            args: &ARGS,
        }
        .render()
        .unwrap(),
    )
}

pub fn list_router() -> axum::Router<AppState> {
    axum::Router::new().route("/list", axum::routing::get(list))
}
