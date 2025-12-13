use crate::components::token;
use actix_web::dev::forward_ready;
use actix_web::error::InternalError;
use actix_web::{
    Error, HttpResponse,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use futures_util::future::{LocalBoxFuture, Ready, ready};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse<'a> {
    message: &'a str,
}

fn unauthorized_error(message: &str) -> Error {
    InternalError::from_response(
        "Unauthorized",
        HttpResponse::Unauthorized().json(ErrorResponse { message }),
    )
    .into()
}

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService { service }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
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
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        let fut = self.service.call(req);

        Box::pin(async move {
            let auth = match auth_header {
                Some(h) if h.starts_with("Bearer ") => h,
                _ => {
                    return Err(unauthorized_error("Unauthorized"));
                }
            };

            let token_data =
                token::verify(auth.to_string().trim_start_matches("Bearer ").to_string());
            let sub = token_data.get("sub");

            if sub.is_none() {
                return Err(unauthorized_error("Unauthorized"));
            }

            let res = fut.await?;

            Ok(res)
        })
    }
}
