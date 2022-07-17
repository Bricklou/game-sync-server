use actix_csrf::{
    extractor::{CsrfCookieConfig, CsrfHeaderConfig},
    CsrfMiddleware,
};
use actix_web::{
    http::{header::HeaderName, Method},
    web::{self, resource, ServiceConfig},
};
use rand::prelude::StdRng;

use super::games;
use super::views;
use super::{auth, middlewares};

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
                    web::scope("/auth")
                        .route("", web::post().to(auth::views::login))
                        .route("", web::get().to(auth::views::refresh))
                        .service(
                            web::resource("")
                                .wrap(middlewares::auth::Auth)
                                .route(web::delete().to(auth::views::logout)),
                        ),
                )
                .service(web::resource("/csrf").route(web::get().to(views::csrf)))
                .service(
                    web::scope("")
                        .service(
                            web::resource("/games")
                                .route(web::post().to(games::views::create_game))
                                .route(web::get().to(games::views::get_games)),
                        )
                        .wrap(middlewares::auth::Auth),
                )
                .wrap(csrf),
        )
        .service(web::resource("{any:(.*)?}").route(web::get().to(views::index)));
}
