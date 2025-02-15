use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;

#[derive(Deserialize,Serialize)]
pub struct Weapon{
    pub id: i32,
    pub name: String,
    pub weight: BigDecimal,
    pub origin: String,
    pub first_use_year: Option<i32>,
    pub type_id: i32,
    pub material_id: i32,
}

impl Weapon {
    pub fn new(
        id: i32, 
        name: String,
        weight: BigDecimal,
        origin: String,
        first_use_year: Option<i32>,
        type_id: i32,
        material_id: i32
     ) -> Self {
        Weapon {
            id: id,
            name: name,
            weight: weight,
            origin: origin,
            first_use_year: first_use_year,
            type_id: type_id,
            material_id: material_id,
        }
    }
}
#[derive(Deserialize,Serialize)]
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