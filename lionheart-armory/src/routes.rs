use axum::{middleware, routing::{get, post}, Router};
use crate::handlers::auth::{login_user, register_user};
use crate::middleware::auth_middleware;


pub fn all_route(database_pool: sqlx::Pool<sqlx::Postgres> ) -> Router {
    Router::new()
    .route("/auth/", get(|| async {"Hello Aldino!"}).route_layer(middleware::from_fn(auth_middleware)))
    .route("/register", post(register_user))
    .route("/login", post(login_user))
    .with_state(database_pool)
}

