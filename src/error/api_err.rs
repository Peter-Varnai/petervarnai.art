use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub struct ApiError(pub ApiErrorKind);

#[derive(Debug)]
pub enum ApiErrorKind {
    JsonError(serde_json::Error),
    SqlError(rusqlite::Error),
    TemplatingError(tera::Error),
    DateFormattingError(time::error::Format),
    MultipartError(actix_multipart::MultipartError),
    FileSystemError(std::io::Error),
}

// From implementations
impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError(ApiErrorKind::JsonError(err))
    }
}

impl From<rusqlite::Error> for ApiError {
    fn from(err: rusqlite::Error) -> Self {
        ApiError(ApiErrorKind::SqlError(err))
    }
}

impl From<tera::Error> for ApiError {
    fn from(err: tera::Error) -> Self {
        ApiError(ApiErrorKind::TemplatingError(err))
    }
}

impl From<time::error::Format> for ApiError {
    fn from(err: time::error::Format) -> Self {
        ApiError(ApiErrorKind::DateFormattingError(err))
    }
}

impl From<actix_multipart::MultipartError> for ApiError {
    fn from(err: actix_multipart::MultipartError) -> Self {
        ApiError(ApiErrorKind::MultipartError(err))
    }
}

impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        ApiError(ApiErrorKind::FileSystemError(err))
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            ApiErrorKind::JsonError(e) => write!(f, "Json serialization error: {}", e),
            ApiErrorKind::SqlError(e) => write!(f, "Database problem: {}", e),
            ApiErrorKind::DateFormattingError(e) => write!(f, "Date formatting problem: {}", e),
            ApiErrorKind::TemplatingError(e) => write!(f, "Template rendering problem: {}", e),
            ApiErrorKind::MultipartError(e) => {
                write!(f, "Multipart form processing problem: {}", e)
            }
            ApiErrorKind::FileSystemError(e) => write!(f, "Error while writing file: {}", e),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match &self.0 {
            ApiErrorKind::JsonError(err) => {
                HttpResponse::InternalServerError().body(format!("Error serializing json! {}", err))
            }
            ApiErrorKind::SqlError(err) => {
                HttpResponse::InternalServerError().body(format!("Database Error! {}", err))
            }
            ApiErrorKind::DateFormattingError(err) => {
                HttpResponse::InternalServerError().body(format!("Date formatting Error! {}", err))
            }
            ApiErrorKind::TemplatingError(err) => HttpResponse::InternalServerError()
                .body(format!("Error rendering template! {}", err)),
            ApiErrorKind::MultipartError(err) => {
                HttpResponse::InternalServerError().body(format!("Error uploading files! {}", err))
            }
            ApiErrorKind::FileSystemError(err) => {
                HttpResponse::InternalServerError().body(format!("Error writing files! {}", err))
            }
        }
    }
}
