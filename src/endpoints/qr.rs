use crate::AppState;
use axum::Router;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use microbin_frontend::components::error::Error;
use models::error_handling::AppError;
use models::pasta::Pasta;
use models::util::animalnumbers::to_u64;
use models::util::hashids::to_u64_hash_ids;
use models::util::misc;
use reqwest::header;

pub async fn getqr(
    State(AppState { args, db }): State<AppState>,
    Path(id): Path<String>,
) -> Result<axum::response::Response, AppError> {
    let u64_id = if args.hash_ids {
        to_u64_hash_ids(&id).unwrap_or(0)
    } else {
        to_u64(&id).unwrap_or(0)
    };

    let opt_pasta = db.get_pasta(&u64_id)?;

    let pasta: Pasta = match opt_pasta {
        Some(pasta) => Ok::<Pasta, AppError>(pasta.into()),
        None => {
            // otherwise, send pasta not found error
            return Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string())],
                dioxus_ssr::render_element(Error(args.into())),
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

    let qr_template = dioxus_ssr::render_element(microbin_frontend::components::qr::QRCode(
        (args, svg, pasta).into(),
    ));

    // serve qr code in template
    Ok([(header::CONTENT_TYPE, "text/html; charset=utf-8")]
        .into_response()
        .map(|_| qr_template)
        .into_response())
}

pub fn qr_router() -> Router<AppState> {
    Router::new().route("/qr/{id}", get(getqr))
}
