use axum::{extract::{Path, State}, http::StatusCode, Json};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;

#[derive(Serialize)]
 pub struct Runestone {
     id: i32,
     name: String,
     material: Option<String>,
     inscription: Option<String>,
     magic_type: Option<String>,
     power_level: Option<i32>,
     discovered_on: Option<NaiveDate>,
     created_at: Option<NaiveDateTime>,
     updated_at: Option<NaiveDateTime>,
     location: Option<String>
}

#[derive(Deserialize)]
pub struct CreateRunestoneRequest{
     name: String,
     material: Option<String>,
     inscription: Option<String>,
     magic_type: Option<String>,
     power_level: Option<i32>,
     discovered_on: Option<NaiveDate>,
     location: Option<String>
}

#[derive(Deserialize)]
pub struct UpdateRunestoneRequest{
     name: String,
     material: Option<String>,
     inscription: Option<String>,
     magic_type: Option<String>,
     power_level: Option<i32>,
     discovered_on: Option<NaiveDate>,
     location: Option<String>
}

#[derive(Serialize)]
pub struct CreateRunestoneRespone{
     id: i32,
}

pub async fn get_all_runestones(State(pg_pool): State<PgPool>) -> Result<(StatusCode,String),(StatusCode,String)>{
    let rows = sqlx::query_as!(Runestone,"SELECT * FROM runestones ORDER BY id")
    .fetch_all(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        )
    })?;
    Ok((
        StatusCode::OK,
        json!({"success": true, "data": rows}).to_string(),
    ))
}

pub async fn create_runestone(
    State(pg_pool): State<PgPool>,
    Json(runestone): Json<CreateRunestoneRequest>
    ) -> Result<(StatusCode,String),(StatusCode,String)> {
        let row = sqlx::query_as!(CreateRunestoneRespone,
            "INSERT INTO runestones (name, material, inscription, magic_type, power_level, discovered_on, location) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id",
            runestone.name,
            runestone.material,
            runestone.inscription,
            runestone.magic_type,
            runestone.power_level,
            runestone.discovered_on,
            runestone.location
        )
        .fetch_one(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;
        Ok((
            StatusCode::OK,
            json!({"success": true, "data": row}).to_string(),
        ))
}

pub async fn get_runestone(
    State(pg_pool): State<PgPool>, 
    Path(id): Path<i32>) 
    -> Result<(StatusCode,String),(StatusCode,String)> 
    {
        let row = sqlx::query_as!(Runestone,"SELECT * FROM runestones WHERE id = $1",id)
        .fetch_one(&pg_pool)
        .await
        .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        )
        })?;
        Ok((
            StatusCode::OK,
            json!({"success": true, "data": row}).to_string(),
        ))
}

pub async fn update_runestone(
    State(pg_pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(runestone): Json<UpdateRunestoneRequest>
    ) -> Result<(StatusCode,String),(StatusCode,String)> {
        sqlx::query!(
            " 
            UPDATE runestones SET
                name = $2,
                material = $3, 
                inscription = $4, 
                magic_type = $5, 
                power_level = $6, 
                discovered_on = $7, 
                location = $8
            WHERE id = $1 ",
            id,
            runestone.name,
            runestone.material,
            runestone.inscription,
            runestone.magic_type,
            runestone.power_level,
            runestone.discovered_on,
            runestone.location
        )
        .execute(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

        Ok((
            StatusCode::OK,
            json!({"success": true}).to_string(),
        ))
}

pub async fn delete_runestone(
    State(pg_pool): State<PgPool>,Path(id): Path<i32>)
    -> Result<(StatusCode,String),(StatusCode,String)> {

        sqlx::query!("DELETE FROM runestones WHERE id = $1", id)
        .execute(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

        Ok((
            StatusCode::OK,
            json!({"success": true}).to_string(),
        ))
}