use actix_web::{Error, HttpRequest, HttpResponse, Result};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header::AUTHORIZATION;
use futures::future::{ok, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::models::user_models::Claims;

#[derive(Clone)]
pub struct AuthMiddleware;

impl<S> actix_service::Transform<S> for AuthMiddleware {
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::Error>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService { service })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S> actix_service::Service for AuthMiddlewareService<S>
where
    S: actix_service::Service,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
            let token = auth_header.to_str().unwrap_or("");
            
            match decode::<Claims>(
                token,
                &DecodingKey::from_secret("your_secret_key".as_ref()),
                &Validation::new(Algorithm::HS256),
            ) {
                Ok(_) => {
                    return self.service.call(req);
                }
                Err(_) => {
                    return ok(req.error_response(HttpResponse::Unauthorized().finish()));
                }
            }
        }

        ok(req.error_response(HttpResponse::Unauthorized().finish()))
    }
}
