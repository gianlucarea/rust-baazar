use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use bcrypt::{hash, verify};
use serde_json::json;
use sqlx::PgPool;
use chrono::Utc;

use crate::models::{RegisterResponse, User};
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