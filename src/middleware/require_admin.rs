use actix_identity::IdentityExt;
use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header::LOCATION,
    Error, HttpResponse,
};
use std::future::Future;
use std::pin::Pin;
use std::task;

pub struct RequireAdmin;

impl<S, B> Transform<S, ServiceRequest> for RequireAdmin
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = RequireAdminMiddleware<S>;
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(RequireAdminMiddleware { service }))
    }
}

pub struct RequireAdminMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequireAdminMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut task::Context<'_>) -> task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let is_authed = req.get_identity().and_then(|id| id.id()).is_ok();

        if is_authed {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_left_body())
            })
        } else {
            Box::pin(async move {
                let resp = HttpResponse::Found()
                    .append_header((LOCATION, "/admin"))
                    .finish()
                    .map_into_right_body();

                Ok(req.into_response(resp))
            })
        }
    }
}
