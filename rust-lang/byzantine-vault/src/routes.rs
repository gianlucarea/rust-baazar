use axum::{middleware, routing::{get, post}, Router};

use crate::{handlers::{login_user, register_user,create_file}, middleware::auth_middleware};


pub fn all_route(database_pool: sqlx::Pool<sqlx::Postgres> ) -> Router {
    Router::new()
    .route("/", get(|| async {"Hello Aldino!"}))
    .route("/register", post(register_user))
    .route("/login", post(login_user))
    .route("/protected", get(|| async {"Hello Logged User!"}).route_layer(middleware::from_fn(auth_middleware)))
    .route("/auth/{owner_id}/files/upload", post(create_file))
    .with_state(database_pool)
}

