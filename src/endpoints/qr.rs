use crate::AppState;
use crate::args::Args;
use crate::endpoints::errors::ErrorTemplate;
use crate::error_handling::AppError;
use crate::pasta::Pasta;
use crate::util::animalnumbers::to_u64;
use crate::util::hashids::to_u64 as hashid_to_u64;
use crate::util::misc::{self};
use askama::Template;
use axum::Router;
use axum::extract::{Path, State};
use axum::http::StatusCode;
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
    State(AppState { args, db }): State<AppState>,
    Path(id): Path<String>,
) -> Result<axum::response::Response, AppError> {
    let u64_id = if args.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let opt_pasta = db.get_pasta(&u64_id)?;

    let mut pasta: Pasta = match opt_pasta {
        Some(pasta) => Ok::<Pasta, AppError>(pasta.into()),
        None => {
            // otherwise, send pasta not found error
            return Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
                ErrorTemplate { args: &args }.render()?,
            )
                .into_response());
        }
    }?;

    // generate the QR code as an SVG - if its a file or text pastas, this will point to the /upload endpoint, otherwise to the /url endpoint, essentially directly taking the user to the url stored in the pasta
    let svg: String = match pasta.pasta_type.as_str() {
        "url" => {
            misc::string_to_qr_svg(format!("{}/url/{}", &args.public_path_as_str(), &id).as_str())
        }
        _ => misc::string_to_qr_svg(
            format!("{}/upload/{}", &args.public_path_as_str(), &id).as_str(),
        ),
    };

    let qr_template = QRTemplate {
        qr: &svg,
        pasta: &pasta,
        args: &args,
    }
    .render()?;

    // serve qr code in template
    Ok([(header::CONTENT_TYPE, "text/html; charset=utf-8")]
        .into_response()
        .map(|_| qr_template)
        .into_response())
}

pub fn qr_router() -> Router<AppState> {
    Router::new().route("/qr/{id}", get(getqr))
}
