use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
 use chrono::{Utc, Duration};
use std::env;

use crate::models::user::Claims;

pub fn generate_token(username: &String) -> Result<String, jsonwebtoken::errors::Error>{
    let secret = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set!");
    let expiration = Utc::now().checked_add_signed(Duration::hours(3)).expect("Invalid timestamp").timestamp() as usize;
    let claims = Claims{sub: username.to_owned(), exp: expiration };
    encode(&Header::default(), & claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set!");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256))?;
    Ok(token_data.claims)
}

