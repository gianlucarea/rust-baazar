use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;


#[derive(Debug,Serialize,Deserialize)]
pub struct User{
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse{
    pub id: Uuid,
}

 
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}