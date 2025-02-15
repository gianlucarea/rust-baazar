use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde_json::json;
use sqlx::PgPool;

use crate::models::weapon::{CreateWeapon, ResultWeaponId, Weapon};
 

pub async fn create_weapon(
    State(pg_pool): State<PgPool>,
    Json(weapon): Json<CreateWeapon>
    ) -> Result<(StatusCode,String),(StatusCode,String)> {
        let row = sqlx::query_as!(ResultWeaponId,
            "INSERT INTO weapons (name, weight, origin, first_use_year, type_id, material_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
            weapon.name,
            weapon.weight,
            weapon.origin,
            weapon.first_use_year,
            weapon.type_id,
            weapon.material_id,
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

pub async fn get_all_weapons(State(pg_pool): State<PgPool>) -> Result<(StatusCode,String),(StatusCode,String)>{
    let rows = sqlx::query_as!(Weapon,"SELECT * FROM weapons ORDER BY id")
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

pub async fn get_weapon(State(pg_pool): State<PgPool>, Path(weapon_id): Path<i32>) -> Result<(StatusCode,String),(StatusCode,String)>{
    let rows = sqlx::query_as!(Weapon,"SELECT * FROM weapons WHERE id = $1", weapon_id)
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
        json!({"success": true, "data": rows}).to_string(),
    ))
}

pub async fn update_weapon(
    State(pg_pool): State<PgPool>,
    Path(weapon_id): Path<i32>,
    Json(weapon): Json<CreateWeapon>
    ) -> Result<(StatusCode,String),(StatusCode,String)> {
        sqlx::query!(
            " 
            UPDATE weapons SET
                name = $2,
                weight = $3, 
                origin = $4, 
                first_use_year = $5, 
                type_id = $6, 
                material_id = $7
            WHERE id = $1 ",
            weapon_id,
            weapon.name,
            weapon.weight,
            weapon.origin,
            weapon.first_use_year,
            weapon.type_id,
            weapon.material_id,
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

pub async fn delete_weapon(
    State(pg_pool): State<PgPool>,Path(weapon_id): Path<i32>)
    -> Result<(StatusCode,String),(StatusCode,String)> {

        sqlx::query!("DELETE FROM weapons WHERE id = $1", weapon_id)
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