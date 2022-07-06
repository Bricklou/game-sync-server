use actix_web::{Error, HttpResponse};

pub type AsyncHttpResponse = Result<HttpResponse, Error>;
pub type ValidatedJson<T> = actix_web_validator::Json<T>;
