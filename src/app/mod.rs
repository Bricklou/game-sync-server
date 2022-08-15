use actix_web::{
    error::{self, JsonPayloadError},
    web, HttpRequest, HttpResponse,
};
use actix_web_validator::{Error, JsonConfig, QueryConfig};
use std::collections::HashMap;

use tera::Tera;

use self::utils::{
    errors::{ErrorHandling, ValidationErrorJsonPayload},
    hash::Hash,
};

mod api;
mod models;
mod panel;
mod types;
pub mod utils;

pub fn register_urls(cfg: &mut web::ServiceConfig) {
    api::urls::register_urls(cfg);
    panel::urls::register_urls(cfg);
}

pub fn setup_templates(cfg: &mut web::ServiceConfig) {
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/views/**/*")).unwrap();

    cfg.app_data(web::Data::new(tera));
}

pub fn setup_data(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::Data::new(Hash::new()));

    cfg.app_data(web::JsonConfig::default().error_handler(json_error_handler));
    configure_errors(cfg);
}

fn configure_errors(cfg: &mut web::ServiceConfig) {
    // Json string default configuration
    cfg.app_data(JsonConfig::default().error_handler(|err, _| {
        let json_error = match &err {
            Error::Validate(error) => ValidationErrorJsonPayload::from(error),
            _ => ValidationErrorJsonPayload {
                message: err.to_string(),
                fields: HashMap::new(),
            },
        };
        error::InternalError::from_response(err, HttpResponse::BadRequest().json(json_error)).into()
    }));

    cfg.app_data(QueryConfig::default().error_handler(|err, _| {
        let json_error = match &err {
            Error::Validate(error) => ValidationErrorJsonPayload::from(error),
            _ => ValidationErrorJsonPayload {
                message: err.to_string(),
                fields: HashMap::new(),
            },
        };
        error::InternalError::from_response(err, HttpResponse::BadRequest().json(json_error)).into()
    }));
}

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    let details = ErrorHandling::new(err.to_string());

    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().json(details),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().json(details)
        }
        _ => HttpResponse::BadRequest().json(details),
    };

    error::InternalError::from_response(err, resp).into()
}
