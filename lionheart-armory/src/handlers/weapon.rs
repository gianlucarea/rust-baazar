use axum::{extract::State, http::StatusCode, Json};
use serde_json::json;
use sqlx::PgPool;

use crate::models::weapon::{CreateWeapon, ResultWeaponId};

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