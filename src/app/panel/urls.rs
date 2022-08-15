use actix_csrf::{
    extractor::{CsrfCookieConfig, CsrfHeaderConfig},
    CsrfMiddleware,
};
use actix_web::{
    http::{header::HeaderName, Method},
    web::{self, ServiceConfig},
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
                .service(auth::url::register_routes())
                .route("/csrf", web::get().to(views::csrf))
                .service(games::url::register_urls().wrap(middlewares::auth::Auth))
                .wrap(csrf),
        )
        .route(
            "/static/{filename:.*}",
            web::get()
                .to(views::static_files)
                .wrap(middlewares::auth::Auth),
        )
        .service(web::resource("{any:(.*)?}").route(web::get().to(views::index)));
}
