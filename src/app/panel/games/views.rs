use actix_csrf::extractor::{Csrf, CsrfHeader};
use actix_web::{dev::Service, web, HttpResponse};

use crate::{
    app::{
        models::game::{Game, GameCreate},
        types::{AsyncHttpResponse, ValidatedJson},
        utils::errors::{ErrorHandling, ServiceError},
    },
    db::DbPool,
};

pub async fn create_game(
    _: Csrf<CsrfHeader>,
    pool: web::Data<DbPool>,
    game: ValidatedJson<GameCreate>,
) -> AsyncHttpResponse {
    let exists = Game::check_if_exist(&game, &pool)
        .await
        .map_err(|e| ServiceError::from(e))?;

    if exists {
        return Err(ServiceError::Conflict.into());
    }

    let game = Game::create(&game, &pool).await;

    match game {
        Ok(g) => Ok(HttpResponse::Ok().json(g)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorHandling::new(e.to_string()))),
    }
}
