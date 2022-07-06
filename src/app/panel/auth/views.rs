use actix_csrf::extractor::{Csrf, CsrfHeader};
use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::{
    app::{
        models::user::User,
        types::{AsyncHttpResponse, ValidatedJson},
        utils::{errors::ErrorHandling, hash::Hash},
    },
    db::DbPool,
};

use super::models::UserLogin;

pub async fn login(
    _: Csrf<CsrfHeader>,
    user: ValidatedJson<UserLogin>,
    pool: web::Data<DbPool>,
    hash: web::Data<Hash>,
    session: Session,
) -> AsyncHttpResponse {
    let logged_user = User::login(&pool, &hash, &user.username, &user.password).await;

    if let Err(e) = logged_user {
        let error_obj = ErrorHandling::new(e.to_string());
        return Ok(HttpResponse::InternalServerError().json(error_obj));
    }

    let obj = logged_user.unwrap();

    if let Some(user) = obj {
        session.insert("user_id", user.id)?;
        Ok(HttpResponse::Ok().json(user))
    } else {
        Ok(HttpResponse::Unauthorized()
            .json(ErrorHandling::new("Invalid user identifiers".to_string())))
    }
}

pub async fn refresh(
    _: Csrf<CsrfHeader>,
    session: Session,
    pool: web::Data<DbPool>,
) -> AsyncHttpResponse {
    let id = session.get("user_id");

    let not_authenticated_response =
        HttpResponse::Unauthorized().json(ErrorHandling::new("User not authenticated".to_string()));

    if let Err(e) = id {
        return Ok(HttpResponse::InternalServerError().json(ErrorHandling::new(
            "Error while getting user session".to_string(),
        )));
    }

    let id = id.unwrap();

    if id == None {
        return Ok(not_authenticated_response);
    }

    let user = User::get_from_id(&pool, id.unwrap()).await;

    match user {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(_) => Ok(not_authenticated_response),
    }
}
