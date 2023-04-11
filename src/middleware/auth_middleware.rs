use actix_web::{dev::ServiceRequest, Error, FromRequest};
use actix_web::dev::{Service, Transform};
use futures::future::{ok, Ready};
use std::task::{Context, Poll};

pub struct AuthMiddleware;

impl<S, B> Transform<S> for AuthMiddleware
where
    S: Service<Request = ServiceRequest, Response = Service<B>, Error = Error>,
    B: actix_web::dev::MessageBody,
{
    type Request = ServiceRequest;
    type Response = Service<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService { service })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service for AuthMiddlewareService<S>
where
    S: Service<Request = ServiceRequest, Response = Service<B>, Error = Error>,
    B: actix_web::dev::MessageBody,
{
    type Request = ServiceRequest;
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        // TODO: ここで認証ロジックを実装する

        self.service.call(req)
    }
}
