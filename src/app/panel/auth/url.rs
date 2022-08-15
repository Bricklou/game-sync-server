use actix_web::web;

use crate::app::panel::middlewares;

use super::views;

pub fn register_routes() -> actix_web::Scope {
    web::scope("/auth")
        .route("", web::post().to(views::login))
        .route("", web::get().to(views::refresh))
        .service(
            web::resource("")
                .wrap(middlewares::auth::Auth)
                .route(web::delete().to(views::logout)),
        )
}
