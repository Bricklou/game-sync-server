use std::collections::HashMap;

use serde::Serialize;

use actix_web::{http, HttpResponse, ResponseError};
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

    #[display(fmt = "Conflict")]
    Conflict,

    #[display(fmt = "Unprocessable Entity: {}", _0)]
    UnprocessableEntity(String),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal server error")
            }
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(ErrorHandling::new(message.to_string()))
            }
            ServiceError::Unauthorized => {
                HttpResponse::Unauthorized().json(ErrorHandling::new("Unauthorized".to_string()))
            }
            ServiceError::NotFound => {
                HttpResponse::NotFound().json(ErrorHandling::new("Not found".to_string()))
            }
            ServiceError::Conflict => {
                HttpResponse::Conflict().json(ErrorHandling::new("Already exists".to_string()))
            }
            ServiceError::UnprocessableEntity(ref message) => {
                HttpResponse::UnprocessableEntity().json(ErrorHandling::new(message.to_string()))
            }
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

impl From<std::io::Error> for ServiceError {
    fn from(_: std::io::Error) -> Self {
        ServiceError::InternalServerError
    }
}

#[derive(Serialize)]
pub struct ValidationErrorJsonPayload {
    pub message: String,
    pub fields: HashMap<String, Vec<String>>,
}

impl From<&validator::ValidationErrors> for ValidationErrorJsonPayload {
    fn from(error: &validator::ValidationErrors) -> Self {
        println!("{:?}", error.field_errors());
        ValidationErrorJsonPayload {
            message: "Validation error".to_owned(),
            fields: error
                .field_errors()
                .iter()
                .map(|(field, errors)| {
                    (
                        field.to_string(),
                        errors.iter().map(|e| e.to_string()).collect(),
                    )
                })
                .collect(),
        }
    }
}
