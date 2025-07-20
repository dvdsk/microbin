use crate::AppState;
use axum::extract::Path;
use axum::http::Response;
use axum::response::IntoResponse;
use axum::Router;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "templates/assets/"]
struct Asset;

async fn static_resources(Path(path): Path<String>) -> impl IntoResponse {
    if let Some(response) = try_embedded(&path) {
        response
    } else {
        Response::builder()
            .status(404)
            .header("Content-Type", "text/plain")
            .body(axum::body::Body::from("Resource not found"))
            .unwrap()
    }
}

// Hilfsfunktion für rust-embed
fn try_embedded(path: &str) -> Option<axum::http::Response<axum::body::Body>> {
    Asset::get(path).map(|content| {
        axum::http::Response::builder()
            .header(
                "Content-Type",
                mime_guess::from_path(path).first_or_octet_stream().as_ref(),
            )
            .body(axum::body::Body::from(content.data.into_owned()))
            .unwrap()
    })
}

pub fn static_resource_router() -> Router<AppState> {
    Router::new().route("/static/{path}", axum::routing::get(static_resources))
}
