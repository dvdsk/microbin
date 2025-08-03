use crate::{db, AppState};
use crate::args::{Args};
use crate::error_handling::AppError;
use crate::pasta::Pasta;
use crate::util::misc::clean_up_expired_pastes;
use askama::Template;
use axum::extract::State;
use axum::response::IntoResponse;
use reqwest::{StatusCode, header};

#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate<'a> {
    pastas: &'a Vec<Pasta>,
    args: &'a Args,
}

pub async fn list(State(AppState{args,pastas, db}): State<AppState>) -> Result<impl IntoResponse, AppError> {
    if args.no_listing {
        return Ok((
            StatusCode::FOUND,
            [(header::LOCATION, format!("{}/", args.public_path_as_str()))],
            "".to_string(),
        ));
    }

    let mut pastas = pastas.lock().expect("no microbin thread should panic");


    // sort pastas in reverse-chronological order of creation time
    pastas.sort_by(|a, b| b.created.cmp(&a.created));

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
        ListTemplate {
            pastas: &pastas,
            args: &args,
        }
        .render()?,
    ))
}

pub fn list_router() -> axum::Router<AppState> {
    axum::Router::new().route("/list", axum::routing::get(list))
}
