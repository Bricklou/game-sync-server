use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};

use crate::app::{types::AsyncHttpResponse, utils::errors::ServiceError};

pub async fn index(tmpl: web::Data<tera::Tera>) -> AsyncHttpResponse {
    let s = tmpl
        .render("index.html", &tera::Context::new())
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn csrf() -> AsyncHttpResponse {
    let resp = HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("{}");
    Ok(resp)
}

pub async fn static_files(req: HttpRequest) -> Result<NamedFile, actix_web::Error> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let path = PathBuf::from("./tmp/storage").join(path);
    println!("{:?}", path);

    let file = NamedFile::open(path).map_err(|_| ServiceError::NotFound)?;

    Ok(file)
}
