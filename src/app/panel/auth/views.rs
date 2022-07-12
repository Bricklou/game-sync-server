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
    let logged_user = User::login(&pool, &hash, &user.username, &user.password).await?;

    if let Some(user) = logged_user {
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
    let id = session.get::<i32>("user_id");

    let not_authenticated_response =
        HttpResponse::Unauthorized().json(ErrorHandling::new("User not authenticated".to_string()));

    let resp = match id {
        Err(_) => HttpResponse::InternalServerError().json(ErrorHandling::new(
            "Error while getting user session".to_string(),
        )),
        Ok(id) => match id {
            None => not_authenticated_response,
            Some(id) => {
                let user = User::get_from_id(&pool, id).await?;

                match user {
                    None => not_authenticated_response,
                    Some(user) => HttpResponse::Ok().json(user),
                }
            }
        },
    };

    Ok(resp)
}

pub async fn logout(session: Session) -> AsyncHttpResponse {
    session.remove("user_id");
    Ok(HttpResponse::Ok().finish())
}
