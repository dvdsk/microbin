use crate::AppState;
use axum::Router;
use axum::extract::Path;
use axum::http::Response;
use axum::response::IntoResponse;
use models::error_handling::AppError;
use reqwest::StatusCode;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "templates/assets/"]
struct Asset;

async fn static_resources(Path(path): Path<String>) -> Result<impl IntoResponse, AppError> {
    if let Ok(response) = try_embedded(&path) {
        Ok(response)
    } else {
        log::warn!("Resource not found: {path}");
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "text/plain")
            .body(axum::body::Body::from("Resource not found"))
            .map_err(AppError::from)
    }
}

// Hilfsfunktion für rust-embed
fn try_embedded(path: &str) -> Result<Response<axum::body::Body>, AppError> {
    match Asset::get(path).map(|content| {
        axum::http::Response::builder()
            .header(
                "Content-Type",
                mime_guess::from_path(path).first_or_octet_stream().as_ref(),
            )
            .body(axum::body::Body::from(content.data.into_owned()))
    }) {
        Some(response) => Ok(response?),
        None => Err(AppError::bad_request(format!("Bad request: {path}"))),
    }
}

pub fn static_resource_router() -> Router<AppState> {
    Router::new().route("/static/{*path}", axum::routing::get(static_resources))
}
