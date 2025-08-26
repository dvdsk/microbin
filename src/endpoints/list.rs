use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use microbin_frontend::components::list::List;
use models::error_handling::AppError;
use models::pasta::Pasta;
use reqwest::{StatusCode, header};

pub async fn list(
    State(AppState { args, db }): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    if args.no_listing {
        return Ok((
            StatusCode::FOUND,
            [(header::LOCATION, format!("{}/", args.public_path_as_str()))],
            "".to_string(),
        ));
    }

    let mut pastas = db
        .find_all_public_pastas()?
        .iter()
        .map(Pasta::from)
        .collect::<Vec<Pasta>>();
    // sort pastas in reverse-chronological order of creation time
    pastas.sort_by(|a, b| b.created.cmp(&a.created));

    let list = dioxus_ssr::render_element(List((args, pastas).into()));

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
        list,
    ))
}

pub fn list_router() -> axum::Router<AppState> {
    axum::Router::new().route("/list", axum::routing::get(list))
}
