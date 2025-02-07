use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::net::TcpListener;

use sqlx::{postgres::PgPoolOptions, PgPool};


#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Unable to access .env file");

    //Enviroment variables
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    let databse_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in the env file");

    //Database Pool
    let database_pool = PgPoolOptions::new()
    .max_connections(8)
    .connect(&databse_url)
    .await
    .expect("Can't connect to database");

    //TCP listener
    let listener = TcpListener::bind(&server_address)
    .await
    .expect("Could not create TCP listener");

    println!("Listeing on {}", listener.local_addr().unwrap());
    
    //Routes
    let app = Router::new()
    .route("/", get(|| async {"Hello Aldino!"}))
    .route("/runestones", get(get_all_runestones).post(create_runestone))
    .route("/runestones/{runestone_id}", get(get_runestone).patch(update_runestone).delete(delete_runestone))
    .with_state(database_pool);

    //Run
    axum::serve(listener,app)
    .await
    .expect("Error serving application");

}

#[derive(Serialize)]
struct Runestone {
    id: i32,
    name: String,
    material: Option<String>,
    inscription: Option<String>,
    magic_type: Option<String>,
    power_level: Option<i32>,
    #[serde(rename="discoveredOn")]
    discovered_on: Option<NaiveDate>,
    #[serde(rename="createdAt")]
    created_at: Option<NaiveDateTime>,
    #[serde(rename="updatedAt")]
    updated_at: Option<NaiveDateTime>,
    location: Option<String>
}

#[derive(Deserialize)]
struct CreateRunestoneRequest{
    name: String,
    material: Option<String>,
    inscription: Option<String>,
    magic_type: Option<String>,
    power_level: Option<i32>,
    #[serde(rename="discoveredOn")]
    discovered_on: Option<NaiveDate>,
    location: Option<String>
}

#[derive(Serialize)]
struct CreateRunestoneRespone{
    id: i32,
}

async fn get_all_runestones(State(pg_pool): State<PgPool>) -> Result<(StatusCode,String),(StatusCode,String)>{
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

async fn create_runestone(
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

async fn get_runestone(State(pg_pool): State<PgPool>) -> Result<(StatusCode,String),(StatusCode,String)> {
    todo!()

}

async fn update_runestone(State(pg_pool): State<PgPool>) -> Result<(StatusCode,String),(StatusCode,String)> {
    todo!()

}

async fn delete_runestone(State(pg_pool): State<PgPool>) -> Result<(StatusCode,String),(StatusCode,String)> {
    todo!()
}