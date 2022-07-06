use actix_web::{error, web, HttpResponse};

use crate::app::types::AsyncHttpResponse;

pub async fn index(tmpl: web::Data<tera::Tera>) -> AsyncHttpResponse {
    let s = tmpl
        .render("index.html", &tera::Context::new())
        .map_err(|e| {
            return error::ErrorInternalServerError(format!("Template error: {}", e.to_string()));
        })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
