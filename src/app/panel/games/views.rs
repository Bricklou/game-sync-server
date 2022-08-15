use actix_csrf::extractor::{Csrf, CsrfHeader};
use actix_multipart::Multipart;
use actix_web::{dev::Service, web, HttpResponse};
use serde::Serialize;
use std::fs;
use uuid::Uuid;

use crate::{
    app::{
        models::{
            game::{Game, GameCreate},
            game_attachment::GameAttachment,
        },
        types::{AsyncHttpResponse, ValidatedJson, ValidatedQuery},
        utils::{
            app_paths::storage_path,
            download,
            errors::{ErrorHandling, ServiceError},
            paginator::PaginatorQuery,
        },
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

pub async fn get_games(
    _: Csrf<CsrfHeader>,
    pool: web::Data<DbPool>,
    qs: ValidatedQuery<PaginatorQuery>,
) -> AsyncHttpResponse {
    let games = Game::get_games(&pool, &qs.into_object()).await;

    match games {
        Ok(g) => Ok(HttpResponse::Ok().json(g)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorHandling::new(e.to_string()))),
    }
}

pub async fn get_game(
    _: Csrf<CsrfHeader>,
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> AsyncHttpResponse {
    let game = Game::get_by_id(id.into_inner(), &pool).await.map_err(|e| {
        log::error!("{:?}", e);
        ServiceError::from(e)
    })?;

    #[derive(Serialize)]
    struct GameWithAttachments {
        #[serde(flatten)]
        game: Game,
        logo: Option<String>,
        screenshots: Vec<GameAttachment>,
    }

    let logo = GameAttachment::get_logo(&pool, &game)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            ServiceError::from(e)
        })?
        .map_or_else(|| None, |l| Some(l.path));

    let screenshots = GameAttachment::get_screenshots(&pool, &game)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            ServiceError::from(e)
        })?;

    let gwa = GameWithAttachments {
        game,
        logo,
        screenshots,
    };

    Ok(HttpResponse::Ok().json(gwa))
}

pub async fn update_game_logo(
    _: Csrf<CsrfHeader>,
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
    mut payload: Multipart,
) -> AsyncHttpResponse {
    let game = Game::get_by_id(id.into_inner(), &pool)
        .await
        .map_err(|e| ServiceError::from(e))?;

    let old_logo = GameAttachment::get_logo(&pool, &game)
        .await
        .map_err(|e| ServiceError::from(e))?;

    let field = download::get_image_field(&mut payload, "image").await?;

    if field.is_none() {
        return Err(ServiceError::BadRequest("file not found".to_string()).into());
    }
    let mut field = field.unwrap();

    let cached_file = download::save_to_cache(&mut field).await.map_err(|e| {
        log::error!("{:?}", e);
        ServiceError::InternalServerError
    })?;

    match imghdr::from_file(&cached_file) {
        Ok(Some(imghdr::Type::Jpeg)) => Ok(()),
        _ => Err(ServiceError::UnprocessableEntity(
            "File is not a JPEG image".to_string(),
        )),
    }?;

    let file_path = format!("images/{}", Uuid::new_v4().to_string() + ".jpeg");

    let tmp_path = format!("{}/{}", storage_path(), &file_path);

    // Move cached file to image path
    web::block(|| {
        fs::rename(cached_file, tmp_path)
            .map_err(|e| {
                log::error!("{:?}", e);
                ServiceError::InternalServerError
            })
            .unwrap();

        if let Some(old_logo) = old_logo {
            fs::remove_file(format!("{}/{}", storage_path(), &old_logo.path))
                .map_err(|e| {
                    log::error!("{:?}", e);
                    ServiceError::InternalServerError
                })
                .unwrap();
        }
    })
    .await?;

    let attachment = GameAttachment::update_logo(&pool, &game, &file_path.as_str()).await?;

    Ok(HttpResponse::Ok().json(attachment))
}
