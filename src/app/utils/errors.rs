use serde::Serialize;

use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Serialize)]
pub struct ErrorHandling {
    pub error: String,
}

impl ErrorHandling {
    pub fn new(text: String) -> Self {
        Self { error: text }
    }
}
#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal server error")]
    InternalServerError,

    #[display(fmt = "Bad request: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "Not found")]
    NotFound,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal server error")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            ServiceError::NotFound => HttpResponse::NotFound().json("Not found"),
        }
    }
}

impl From<argon2::Error> for ServiceError {
    fn from(_: argon2::Error) -> Self {
        ServiceError::InternalServerError
    }
}

impl From<argon2::password_hash::Error> for ServiceError {
    fn from(_: argon2::password_hash::Error) -> Self {
        ServiceError::InternalServerError
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => ServiceError::NotFound,
            _ => ServiceError::InternalServerError,
        }
    }
}
/*
#[derive(Serialize)]
struct ValidationErrorJsonPayload {
    pub message: String,
    pub fields: Vec<String>,
}

impl From<&validator::ValidationError> for ServiceError {
    fn from(error: &validator::ValidationError) -> Self {
        ServiceError::BadRequest(
            ValidationErrorJsonPayload {
                message: error.to_string(),
                fields: error
                    .field_errors()
                    .iter()
                    .map(|e| e.field_name.to_string())
                    .collect(),
            }
            .to_string(),
        )
    }
}
*/
