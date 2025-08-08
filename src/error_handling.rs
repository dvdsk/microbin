use crate::pasta::Pasta;
use axum::extract::multipart::MultipartError;
use axum::http;
use axum::response::{IntoResponse, Response};
use magic_crypt::MagicCryptError;
use reqwest::StatusCode;
use reqwest::header::InvalidHeaderValue;
use std::fmt::Display;
use std::str::Utf8Error;
use std::sync::{MutexGuard, PoisonError};
use db::db::error::DBError;

#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub code: StatusCode,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppError: {} (code: {})", self.message, self.code)
    }
}

impl AppError {
    pub fn bad_request(message: impl Into<String>) -> AppError {
        AppError {
            message: message.into(),
            code: StatusCode::BAD_REQUEST,
        }
    }
}


impl From<DBError> for AppError {
    fn from(error: DBError) -> Self {
        log::warn!("Database error: {error}");
        AppError {
            message: "An error occurred while accessing the database. Please check the server logs \
            if you are the server admin"
                .to_string(),
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<MagicCryptError> for AppError {
    fn from(error: MagicCryptError) -> Self {
        log::warn!("MagicCrypt error: {error}");
        AppError {
            message: "Encryption/Decryption error. Please check the server logs if you are the \
            server admin"
                .to_string(),
            code: StatusCode::BAD_REQUEST,
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        log::warn!("Request error during fetch: {error}");
        AppError {
            message: "An error occurred while processing your request. Please check the server \
            logs if you are the server admin"
                .to_string(),
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<Utf8Error> for AppError {
    fn from(error: Utf8Error) -> Self {
        log::warn!("UTF-8 error: {error}");
        AppError {
            message: "Invalid UTF-8 sequence. Please check the server logs if you are the \
            server admin"
                .to_string(),
            code: StatusCode::BAD_REQUEST,
        }
    }
}

impl From<InvalidHeaderValue> for AppError {
    fn from(error: InvalidHeaderValue) -> Self {
        log::warn!("Invalid header value: {error}");
        AppError {
            message: "Invalid header value. Please check the server logs if you are the \
            server admin"
                .to_string(),
            code: StatusCode::BAD_REQUEST,
        }
    }
}

impl From<askama::Error> for AppError {
    fn from(error: askama::Error) -> Self {
        AppError {
            message: format!("Template rendering error: {error}"),
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<PoisonError<MutexGuard<'_, Vec<Pasta>>>> for AppError {
    fn from(error: PoisonError<MutexGuard<Vec<Pasta>>>) -> Self {
        AppError {
            message: format!("Mutex error: {error}"),
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
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

impl From<http::Error> for AppError {
    fn from(error: http::Error) -> Self {
        AppError {
            message: format!("HTTP error: {error}"),
            code: StatusCode::INTERNAL_SERVER_ERROR,
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
