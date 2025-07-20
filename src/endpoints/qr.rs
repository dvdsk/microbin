use crate::args::{Args, ARGS};
use crate::endpoints::errors::ErrorTemplate;
use crate::error_handling::AppError;
use crate::pasta::Pasta;
use crate::util::animalnumbers::to_u64;
use crate::util::hashids::to_u64 as hashid_to_u64;
use crate::util::misc::{self, remove_expired};
use crate::AppState;
use askama::Template;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use reqwest::header;

#[derive(Template)]
#[template(path = "qr.html", escape = "none")]
struct QRTemplate<'a> {
    qr: &'a String,
    pasta: &'a Pasta,
    args: &'a Args,
}

pub async fn getqr(
    State(data): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    let u64_id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    // remove expired pastas (including this one if needed)
    remove_expired(&mut pastas);

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
                format!("{}/url/{}", &ARGS.public_path_as_str(), &id).as_str(),
            ),
            _ => misc::string_to_qr_svg(
                format!("{}/upload/{}", &ARGS.public_path_as_str(), &id).as_str(),
            ),
        };

        // serve qr code in template
        return Ok([(header::CONTENT_TYPE, "text/html; charset=utf-8")]
            .into_response()
            .map(|_| {
                QRTemplate {
                    qr: &svg,
                    pasta: &pastas[index],
                    args: &ARGS,
                }
                .render()
                .unwrap()
            }));
    }

    // otherwise
    // send pasta not found error
    Ok([(header::CONTENT_TYPE, "text/html; charset=utf-8")]
        .into_response()
        .map(|_| ErrorTemplate { args: &ARGS }.render().unwrap()))
}

pub fn qr_router() -> Router<AppState> {
    Router::new().route("/qr/{id}", get(getqr))
}
