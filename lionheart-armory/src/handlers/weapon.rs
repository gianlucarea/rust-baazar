use axum::{extract::{Path, State}, http::StatusCode, Json};
use redis::{Commands, Connection};
use serde_json::json;
use sqlx::PgPool;

use crate::models::weapon::{CreateWeapon, ResultWeaponId, Weapon};
 
async fn get_redis_client() -> redis::Connection {
    let client = redis::Client::open("redis://127.0.0.1/").expect("Invalid Redis URL");
    client.get_connection().unwrap()
}

pub async fn create_weapon(
    State(pg_pool): State<PgPool>,
    Json(weapon): Json<CreateWeapon>
    ) -> Result<(StatusCode,String),(StatusCode,String)> {

        let mut con = get_redis_client().await;
        
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
        .await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

        let weapon = Weapon::new(row.id, weapon.name, weapon.weight,  weapon.origin,  weapon.first_use_year,  weapon.type_id,  weapon.material_id);
        let weapon_json = serde_json::to_string(&weapon).unwrap();
        let _: () = con.set(format!("weapon:{}", row.id), weapon_json).unwrap();
        let _: () = con.del("all_weapons").unwrap(); // Invalidate cache
        
        Ok((StatusCode::OK, json!({"success":true, "weapon": row.id}).to_string()))
        
}

pub async fn get_all_weapons(State(pg_pool): State<PgPool>) -> Result<(StatusCode,String),(StatusCode,String)>{
    
    let mut con = get_redis_client().await;

    let cached_weapons: Option<String> = con.get("all_weapons").unwrap();
    
    if let Some(json_data) = cached_weapons {
        println!("Cache hit! Returning cached data.");
        Ok((StatusCode::OK,json_data))
    } else {
        fetch_all_weapons_from_db(pg_pool, &mut con).await
    }
}

async fn fetch_all_weapons_from_db( pg_pool :sqlx::Pool<sqlx::Postgres>, con: &mut Connection ) -> Result<(StatusCode,String),(StatusCode,String)>{
    println!("Fetched All From DB");

    let weapons = sqlx::query_as!(Weapon,"SELECT * FROM weapons ORDER BY id")
    .fetch_all(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        )
    })?;

    let json_swords = serde_json::to_string(&weapons).unwrap();
    let _: () = con.set_ex("all_weapons", json_swords, 60).unwrap();

    Ok((
        StatusCode::OK,
        json!({"success": true, "data": weapons}).to_string(),
    ))
}

pub async fn get_weapon(State(pg_pool): State<PgPool>,Path(weapon_id): Path<i32>) -> Result<(StatusCode,String),(StatusCode,String)>{
    
    let mut con = get_redis_client().await;

    let weapon_json: Option<String> = con.get(format!("weapon:{}",weapon_id)).unwrap();
    
    if let Some(json_data) = weapon_json {
        println!("Fetched From Redis");
        Ok((StatusCode::OK,json_data))
    } else {
        fetch_weapon_from_db(pg_pool, weapon_id,&mut con).await
    }
}

async fn fetch_weapon_from_db( pg_pool: sqlx::Pool<sqlx::Postgres> ,weapon_id: i32, con: &mut Connection) -> Result<(StatusCode,String),(StatusCode,String)>{
    println!("Fetched From DB");
    
    let weapon = sqlx::query_as!(Weapon,"SELECT * FROM weapons WHERE id = $1", weapon_id)
    .fetch_one(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        )
    })?;

    let weapon_json = serde_json::to_string(&weapon).unwrap();
    let _: () = con.set(format!("weapon:{}", weapon.id), weapon_json).unwrap();

    Ok((
        StatusCode::OK,
        json!({"success": true, "data": weapon}).to_string(),
    ))
}

pub async fn update_weapon(
    State(pg_pool): State<PgPool>,
    Path(weapon_id): Path<i32>,
    Json(weapon): Json<CreateWeapon>
    ) -> Result<(StatusCode,String),(StatusCode,String)> {

        let mut con = get_redis_client().await;

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

        let weapon = Weapon::new(weapon_id, weapon.name, weapon.weight,  weapon.origin,  weapon.first_use_year,  weapon.type_id,  weapon.material_id);
        let weapon_json = serde_json::to_string(&weapon).unwrap();
        let _: () = con.set(format!("weapon:{}", weapon_id), weapon_json).unwrap();
        let _: () = con.del("all_weapons").unwrap(); // Invalidate cache

        Ok((
            StatusCode::OK,
            json!({"success": true}).to_string(),
        ))
}

pub async fn delete_weapon(
    State(pg_pool): State<PgPool>,Path(weapon_id): Path<i32>)
    -> Result<(StatusCode,String),(StatusCode,String)> {

        let mut con = get_redis_client().await;

        sqlx::query!("DELETE FROM weapons WHERE id = $1", weapon_id)
        .execute(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

        let _: () = con.del(format!("weapon:{}", weapon_id)).unwrap();
        let _: () = con.del("all_weapons").unwrap(); // Invalidate cache

        Ok((
            StatusCode::OK,
            json!({"success": true}).to_string(),
        ))
}