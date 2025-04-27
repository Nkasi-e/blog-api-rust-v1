use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::AUTHORIZATION,
    middleware::Next,
    Error, HttpResponse,
};

use crate::utils::jwt::decode_jwt;

pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth = req.headers().get(AUTHORIZATION);
    if auth.is_none() {
        // Create an HttpResponse with 401 status
        let response = HttpResponse::Unauthorized()
            .body("Unauthorised");
        // Convert it to an Error that the middleware can return
        return Err(actix_web::error::InternalError::from_response(
            "Unauthorised",
            response,
        )
        .into());
    }

    let token = auth.unwrap().to_str().unwrap().trim_start_matches("Bearer ").to_owned();
    match decode_jwt(token) {
        Ok(claim) => {
            // You can store claim in request extensions if needed
            // req.extensions_mut().insert(claim);
            next.call(req).await
        }
        Err(_) => {
            let response = HttpResponse::Unauthorized()
                .body("Invalid token");
            Err(actix_web::error::InternalError::from_response(
                "Invalid token",
                response,
            )
            .into())
        }
    }
}