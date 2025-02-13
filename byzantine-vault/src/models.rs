use chrono::{DateTime, Utc};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct File{
    pub id: Uuid,
    pub owner_id: Option<Uuid>,
    pub filename: String,
    pub version: i32,
    pub encrypted_data: Vec<u8>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintFile{
    pub id: Uuid,
    pub filename: String,
    pub version: i32,
    pub content: String,
}

impl PrintFile{
    pub fn new(id: Uuid, filename: String, version: i32, content: String) -> Self {
        Self {
            id,
            filename,
            version,
            content,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileResponse{
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileExist{
    pub version: i32,
}

