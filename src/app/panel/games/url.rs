use actix_web::web;

use super::views;

pub fn register_urls() -> actix_web::Scope {
    web::scope("")
        .service(
            web::resource("/games")
                .route(web::post().to(views::create_game))
                .route(web::get().to(views::get_games)),
        )
        .service(
            web::scope("/games/{id}")
                .service(web::resource("").route(web::get().to(views::get_game)))
                .route("/logo", web::put().to(views::update_game_logo)),
        )
}
