use crate::AppState;
use crate::args::Args;
use crate::endpoints::errors::ErrorTemplate;
use crate::error_handling::AppError;
use crate::pasta::Pasta;
use crate::util::animalnumbers::to_u64;
use crate::util::hashids::to_u64 as hashid_to_u64;
use crate::util::misc::{self, clean_up_expired_pastes};
use askama::Template;
use axum::Router;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use reqwest::header;

#[derive(Template)]
#[template(path = "qr.html", escape = "none")]
struct QRTemplate<'a> {
    args: &'a Args,
    qr: &'a String,
    pasta: &'a Pasta,
}

pub async fn getqr(
    State(AppState { pastas, args }): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection
    let mut pastas = pastas.lock().expect("no microbin thread should panic");

    let u64_id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    // remove expired pastas (including this one if needed)
    clean_up_expired_pastes(&mut pastas, &args);

    // find the index of the pasta in the collection based on u64 id
    let mut index: usize = 0;
    let mut found: bool = false;
    for (i, pasta) in pastas.iter().enumerate() {
        if pasta.id == u64_id {
            index = i;
            found = true;
            break;
        }
    }

    if found {
        // generate the QR code as an SVG - if its a file or text pastas, this will point to the /upload endpoint, otherwise to the /url endpoint, essentially directly taking the user to the url stored in the pasta
        let svg: String = match pastas[index].pasta_type.as_str() {
            "url" => misc::string_to_qr_svg(
                format!("{}/url/{}", &args.public_path_as_str(), &id).as_str(),
            ),
            _ => misc::string_to_qr_svg(
                format!("{}/upload/{}", &args.public_path_as_str(), &id).as_str(),
            ),
        };

        let qr_template = QRTemplate {
            qr: &svg,
            pasta: &pastas[index],
            args: &args,
        }
        .render()?;

        // serve qr code in template
        return Ok([(header::CONTENT_TYPE, "text/html; charset=utf-8")]
            .into_response()
            .map(|_| qr_template));
    }

    // otherwise,
    // send pasta not found error
    let err_template = ErrorTemplate { args: &args }.render()?;
    Ok([(header::CONTENT_TYPE, "text/html; charset=utf-8")]
        .into_response()
        .map(|_| err_template))
}

pub fn qr_router() -> Router<AppState> {
    Router::new().route("/qr/{id}", get(getqr))
}
