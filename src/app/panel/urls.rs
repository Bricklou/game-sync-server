use actix_csrf::{
    extractor::{CsrfCookieConfig, CsrfHeaderConfig},
    CsrfMiddleware,
};
use actix_web::{
    http::{header::HeaderName, Method},
    web::{self, ServiceConfig},
};
use rand::prelude::StdRng;

use super::auth;
use super::games;
use super::views;

pub fn register_urls(cfg: &mut ServiceConfig) {
    let csrf = CsrfMiddleware::<StdRng>::new()
        .cookie_name("save-sync-csrf")
        .http_only(false)
        .set_cookie(Method::GET, "/api/csrf");
    let csrf_cookie_config = CsrfCookieConfig::new("save-sync-csrf".to_string());
    let csrf_header_config = CsrfHeaderConfig::new(HeaderName::from_static("x-xsrf-token"));

    cfg.app_data(csrf_cookie_config)
        .app_data(csrf_header_config)
        .service(
            web::scope("/api")
                .service(
                    web::resource("/auth")
                        .route(web::post().to(auth::views::login))
                        .route(web::get().to(auth::views::refresh))
                        .route(web::delete().to(auth::views::logout)),
                )
                .service(web::resource("/games").route(web::post().to(games::views::create_game)))
                .service(web::resource("/csrf").route(web::get().to(views::csrf)))
                .wrap(csrf),
        )
        .service(web::resource("{any:(.*)?}").route(web::get().to(views::index)));
}
