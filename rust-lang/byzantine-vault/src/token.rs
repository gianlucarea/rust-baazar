use jsonwebtoken::{encode, Header, EncodingKey};
 use chrono::{Utc, Duration};
use std::env;

use crate::models::Claims;

pub fn generate_token(username: &String) -> Result<String, jsonwebtoken::errors::Error>{
    let secret = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set!");
    let expiration = Utc::now().checked_add_signed(Duration::hours(3)).expect("Invalid timestamp").timestamp() as usize;
    let claims = Claims{sub: username.to_owned(), exp: expiration };
    encode(&Header::default(), & claims, &EncodingKey::from_secret(secret.as_ref()))
}
