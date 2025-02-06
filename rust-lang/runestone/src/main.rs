use axum::{extract::State, http::StatusCode, routing::get, Router};
use chrono::{Date, DateTime, Utc};
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
    .route("runestones", get(get_all_runestones).post(create_runestone))
    .route("runestones/{runestone_id}", get(get_runestone).patch(update_runestone).delete(delete_runestone))
    .with_state(database_pool);

    //Run
    axum::serve(listener,app)
    .await
    .expect("Error serving application");

}

struct Runestone {
    id: i32,
    name: String,
    material: Option<String>,
    inscription: Option<String>,
    magic_type: Option<String>,
    power_level: Option<i32>,
    discovered_on: Option<DateTime<Utc>>,
    location: Option<String>
}

async fn get_all_runestones(State(pg_pool): State<PgPool>) -> Result<(StatusCode,String),(StatusCode,String)>{
    todo!()
}

async fn create_runestone(State(pg_pool): State<PgPool>) -> Result<(StatusCode,String),(StatusCode,String)> {
    todo!()

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