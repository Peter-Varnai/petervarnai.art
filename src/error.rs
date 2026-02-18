use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database problem: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Serialization problem: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Template rendering problem: {0}")]
    Tera(#[from] tera::Error),

    #[error("Date formatting problem: {0}")]
    DateFormat(#[from] time::error::Format),

    #[error("Multipart form processing problem: {0}")]
    Multipart(#[from] actix_multipart::MultipartError),

    #[error("FileSystem error: {0}")]
    FileSystem(#[from] std::io::Error),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Database(rusqlite::Error::QueryReturnedNoRows) => {
                HttpResponse::NotFound().body("Couldn't find that project!")
            }
            _ => HttpResponse::InternalServerError().body(self.to_string()),
        }
    }
}
