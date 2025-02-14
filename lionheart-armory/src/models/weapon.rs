use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;

#[derive(Deserialize)]
pub struct CreateWeapon{
    pub name: String,
    pub weight: BigDecimal,
    pub origin: String,
    pub first_use_year: Option<i32>,
    pub type_id: i32,
    pub material_id: i32,
}

#[derive(Serialize)]
pub struct ResultWeaponId{
    pub id: i32
}