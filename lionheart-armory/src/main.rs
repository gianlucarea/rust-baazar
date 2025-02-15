use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod routes;
mod token;
mod handlers {
    pub mod auth;
    pub mod weapon;
}
mod models{
    pub mod user;
    pub mod weapon;
}
mod middleware;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Unable to access .env file");
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    let databse_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in the env file");

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
    
    println!("Listeing 🎧 on {}", listener.local_addr().unwrap());
    
    //Routes
    let app = routes::all_route(database_pool);

    axum::serve(listener,app)
    .await
    .expect("Error serving application");
}