use std::env;

use aes_gcm::aead::consts::U12;
use aes_gcm::aead::Aead;
use axum::extract::Path;
use axum::{extract::{State,Multipart}, http::StatusCode, Json};

use bcrypt::{hash, verify};
use chrono::Utc;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce}; 

use crate::models::{File, FileExist, FileResponse, PrintFile, RegisterResponse, User};
use crate::token::generate_token;

pub async fn register_user(
    State(pool): State<PgPool>,
    Json(user):Json<User>) 
    -> (StatusCode, String) {
    
    let hashed_password = match hash(&user.password, bcrypt::DEFAULT_COST){
        Ok(hp) => hp,
        Err(_) => return(StatusCode::INTERNAL_SERVER_ERROR, "Error hashing password".to_string())
    };

    let user_id = sqlx::query_as!(RegisterResponse,
        "INSERT INTO users(username,password_hash,role,created_at,updated_at) VALUES ($1,$2,$3,$4,$5) RETURNING id",
        user.username,
        hashed_password,
        String::from("User"),
        Utc::now(),
        Utc::now(),
    ).fetch_one(&pool).await;


    match user_id {
        Ok(_) => (StatusCode::OK, json!({"success": true, "data": user_id.unwrap().id , "message": "User registered successfully"}).to_string()),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error registering user".into()),
    }
}

pub async fn login_user(
    State(pool): State<PgPool>,
    Json(user): Json<User>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let result = sqlx::query!(
        "SELECT username, password_hash FROM users WHERE username = $1",
        user.username
    )
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(Some(db_user)) => {
            if verify(&user.password, &db_user.password_hash).unwrap_or(false) {
                match generate_token(&db_user.username) {
                    Ok(token) => Ok(Json(serde_json::json!({ "token" : token }))),
                    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

fn get_encryption_var() -> (Aes256Gcm, Nonce<U12>) {
    let key_str = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY not set in .env");
    let nonce_str = env::var("NONCE").expect("NONCE not set in .env");

    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_str.as_bytes());

    (cipher, *nonce)
}

pub async fn create_file(pool:  State<PgPool>, 
    Path(owner_id): Path<Uuid>, 
    mut multipart: Multipart) 
    -> Result<(StatusCode,String),(StatusCode,String)> {

    match multipart.next_field().await {
        Ok(Some(field)) => {
            let filename = field.name().unwrap().to_string();
            let text: String = field.text().await.unwrap().to_owned();
            let text: &[u8] = text.as_bytes(); 
            let version = get_latest_file_version(pool.clone(),&filename,owner_id).await;
            if version != -1 {
                return upload_file(pool,filename,text,owner_id,version + 1).await; 
            } else {
                return upload_file(pool,filename,text,owner_id,0).await; 
            }
        }
        Ok(None) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success":false, "message": "No file found!"}).to_string(),
                ))
        }
        Err(e) => {
            return Err((  
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success":false, "message":e.to_string()}).to_string(),
            ))
        }
    }
}

async fn upload_file(State(pool): State<PgPool>, filename: String ,text: &[u8], owner_id: Uuid, version: i32)-> Result<(StatusCode,String),(StatusCode,String)>{
    let (cipher,nonce) = get_encryption_var();
    let encrypted_data = cipher.encrypt(&nonce, text).expect("Encryption failed");
     
    let data  =sqlx::query_as!(FileResponse,
        "INSERT INTO files (owner_id, filename, version,encrypted_data) VALUES ($1, $2, $3, $4) RETURNING id",
        owner_id, 
        filename,
        version,
        encrypted_data
    ).fetch_one(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success":false, "message":e.to_string()}).to_string(),
        )
    })?;
    Ok((
        StatusCode::OK,
        json!({"success":true, "message": data }).to_string(),
    )) 
}

pub async fn download_file(State(pool): State<PgPool>,Path((owner_id,filename)): Path<(Uuid,String)>)-> Result<(StatusCode,String),(StatusCode,String)>{
    let file = sqlx::query_as!(File,
    "SELECT * FROM files WHERE filename = $1 AND owner_id = $2 ORDER BY version DESC LIMIT 1", 
    filename, 
    owner_id
    ).fetch_one(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success":false, "message":e.to_string()}).to_string(),
        )
    })?;

    let (cipher,nonce) = get_encryption_var();

    let encrypted_data = file.encrypted_data;
    let decrypted_file = cipher.decrypt(&nonce,encrypted_data.as_ref()).expect("Decryption Fail");
    let content : String = String::from_utf8(decrypted_file).unwrap();
    let file: PrintFile = PrintFile::new(file.id, filename, file.version, content);

    Ok((
        StatusCode::OK,
        json!({"success":true, "file": file }).to_string(),
    ))
}

pub async fn download_file_by_version(State(pool): State<PgPool>,Path((owner_id,filename, version)): Path<(Uuid,String,i32)>)-> Result<(StatusCode,String),(StatusCode,String)>{
    let file = sqlx::query_as!(File,
    "SELECT * FROM files WHERE filename = $1 AND owner_id = $2 and version = $3 ORDER BY version DESC LIMIT 1", 
    filename, 
    owner_id,
    version
    ).fetch_one(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success":false, "message":e.to_string()}).to_string(),
        )
    })?;

    let (cipher,nonce) = get_encryption_var();

    let encrypted_data = file.encrypted_data;
    let decrypted_file = cipher.decrypt(&nonce,encrypted_data.as_ref()).expect("Decryption Fail");
    let content : String = String::from_utf8(decrypted_file).unwrap();
    let file: PrintFile = PrintFile::new(file.id, filename, version, content);

    Ok((
        StatusCode::OK,
        json!({"success":true, "file": file }).to_string(),
    ))
}


async fn get_latest_file_version(State(pool): State<PgPool>, filename: &String , owner_id: Uuid)-> i32{
    let data  =sqlx::query_as!(FileExist,
        "SELECT version FROM files WHERE filename=$1 AND owner_id=$2 ORDER BY version DESC",filename,owner_id, 
    ).fetch_one(&pool)
    .await;
    if data.is_ok() {
        data.unwrap().version
    } else {
        -1
    }
}

