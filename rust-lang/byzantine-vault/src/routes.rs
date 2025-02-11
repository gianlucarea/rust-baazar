use axum::{routing::{get, post}, Router};

use crate::handlers::{login_user, register_user};


pub fn all_route(database_pool: sqlx::Pool<sqlx::Postgres> ) -> Router {
    Router::new()
    .route("/", get(|| async {"Hello Aldino!"}))
    .route("/register", post(register_user))
    .route("/login", post(login_user))
    .with_state(database_pool)
}

