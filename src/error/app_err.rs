use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub struct AppError(pub AppErrorKind);

#[derive(Debug)]
pub enum AppErrorKind {
    SqlError(rusqlite::Error),
    JsonError(serde_json::Error),
    TeraError(tera::Error),
}

// Display impl for AppError
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            AppErrorKind::SqlError(e) => write!(f, "Database problem: {}", e),
            AppErrorKind::JsonError(e) => write!(f, "Serialization problem: {}", e),
            AppErrorKind::TeraError(e) => write!(f, "Template rendering problem: {}", e),
        }
    }
}

// ResponseError impl for Actix
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match &self.0 {
            AppErrorKind::SqlError(rusqlite::Error::QueryReturnedNoRows) => {
                HttpResponse::NotFound().body("Couldn't find that project!")
            }
            AppErrorKind::SqlError(err) => HttpResponse::InternalServerError()
                .body(format!("Oops! Database trouble \n{}", err)),
            AppErrorKind::JsonError(_) => {
                HttpResponse::InternalServerError().body("Internal Server Error")
            }
            AppErrorKind::TeraError(_) => {
                HttpResponse::InternalServerError().body("Template rendering error")
            }
        }
    }
}

// From implementations
impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError(AppErrorKind::SqlError(err))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError(AppErrorKind::JsonError(err))
    }
}

impl From<tera::Error> for AppError {
    fn from(err: tera::Error) -> Self {
        AppError(AppErrorKind::TeraError(err))
    }
}
