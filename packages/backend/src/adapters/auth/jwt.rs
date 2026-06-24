use crate::models::entities::UserId;
use crate::ports::auth::TokenProvider;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use worker::{Date, Error, Result};

#[derive(Clone)]
pub struct JwtProvider {
    secret: String,
}

impl JwtProvider {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl TokenProvider for JwtProvider {
    fn generate_token(&self, user_id: &UserId) -> Result<String> {
        let exp = (Date::now().as_millis() / 1000) as usize + 7200; // 2 hours
        let claims = Claims {
            sub: user_id.0.clone(),
            exp,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|e| Error::from(format!("Failed to encode JWT: {}", e)))
    }

    fn verify_token(&self, token: &str) -> Result<UserId> {
        let decoding_key = DecodingKey::from_secret(self.secret.as_ref());
        let validation = Validation::default();

        decode::<Claims>(token, &decoding_key, &validation)
            .map(|data| UserId(data.claims.sub))
            .map_err(|e| Error::from(format!("JWT verification failed: {}", e)))
    }
}
