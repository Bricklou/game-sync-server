use std::future::{ready, Ready};

use actix_session::SessionExt;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::{future::LocalBoxFuture, FutureExt};

use crate::{app::utils::errors::ErrorHandling, db::DbPool};

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();
        let pool = req.app_data::<DbPool>().unwrap();
        let id = session.get::<i32>("user_id");

        /*async move {
            let not_authenticated_response = HttpResponse::Unauthorized()
                .json(ErrorHandling::new("User not authenticated".to_string()));

            let fut = self.service.call(req);
            if let Err(_) = id {
                return ready(Ok(req.into_response(not_authenticated_response)));
            }

            let id = id.unwrap();

            if id == None {
                return Box::pin(fut);
            }

            let res = fut.await?;

            let user = User::get_from_id(&pool, id.unwrap()).await;

            if user.is_err() {
                return ready(Ok(HttpResponse::Unauthorized()
                    .json(ErrorHandling::new("Invalid user identifiers".to_string()))));
            }

            Ok(res)
        }
        .boxed_local()*/

        let fut = self.service.call(req);

        async move {
            let not_authenticated_response = HttpResponse::Unauthorized()
                .json(ErrorHandling::new("User not authenticated".to_string()));

            if let Err(_) = id {
                //   return ready(Ok(req.into_response(not_authenticated_response)));
            }

            let res = fut.await?;
            Ok(res)
        }
        .boxed_local()
    }
}
