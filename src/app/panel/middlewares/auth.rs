use actix_session::SessionExt;
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::rc::Rc;

use crate::app::utils::errors::ErrorHandling;

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();

        let user_id = session.get::<i32>("user_id");

        match user_id {
            Err(e) => {
                log::error!("{}", e);
                let (request, _pl) = req.into_parts();

                let res = HttpResponse::InternalServerError()
                    .json(ErrorHandling::new(format!(
                        "Error while getting user session: {}",
                        e
                    )))
                    .map_into_right_body();
                Box::pin(async move { Ok(ServiceResponse::new(request, res)) })
            }
            Ok(user_id) => match user_id {
                None => {
                    let (request, _pl) = req.into_parts();

                    let res = HttpResponse::Unauthorized()
                        .json(ErrorHandling::new("User not authenticated".to_string()))
                        .map_into_right_body();
                    Box::pin(async { Ok(ServiceResponse::new(request, res)) })
                }
                Some(_) => {
                    let res = self.service.call(req);
                    Box::pin(async move {
                        // forwarded responses map to "left" body
                        res.await.map(ServiceResponse::map_into_left_body)
                    })
                }
            },
        }
    }
}
