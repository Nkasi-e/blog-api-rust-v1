use std::future::{ready, Ready};
use actix_web::{FromRequest, HttpMessage};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use super::constants;

// Data stored in our jwt
#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: usize, // expiry time for the token
    pub iat: usize, // time the token was created
    pub email: String,
    pub id: i32,
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        match req.extensions().get::<Claims>() {
            Some(claim) => ready(Ok(claim.clone())),
            None => ready(Err(actix_web::error::ErrorBadRequest("Bad Request Token"))),
        }
    }
}

// encoding jwt
pub fn encode_jwt(email: String, id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expire = Duration::hours(24);

    let claims = Claims {
        exp: (now + expire).timestamp() as usize,
        iat: now.timestamp() as usize,
        email,
        id
    };

    let secret = (*constants::SECRET).clone(); // used * to dereference

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}


// decode jwt
pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>,jsonwebtoken::errors::Error> {
    let secret = (*constants::SECRET).clone();
    let claim_data: Result<TokenData<Claims>, jsonwebtoken::errors::Error> = decode(&jwt, 
        &DecodingKey::from_secret(secret.as_ref()), 
        &Validation::default()
    );

    claim_data
}