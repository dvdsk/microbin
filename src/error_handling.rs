use axum::extract::multipart::MultipartError;
use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;

#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub code: StatusCode,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = self.message;
        (self.code, body).into_response()
    }
}

impl From<MultipartError> for AppError {
    fn from(error: MultipartError) -> Self {
        AppError {
            message: format!("Multipart error: {error}"),
            code: StatusCode::BAD_REQUEST,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError {
            message: format!("IO error: {error}"),
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
