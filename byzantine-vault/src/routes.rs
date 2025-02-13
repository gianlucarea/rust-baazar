use axum::{middleware, routing::{get, post}, Router};

use crate::{handlers::{login_user, register_user,create_file,download_file,download_file_by_version,delete_file,delete_file_by_version}, middleware::auth_middleware};


pub fn all_route(database_pool: sqlx::Pool<sqlx::Postgres> ) -> Router {
    Router::new()
    .route("/", get(|| async {"Hello Aldino!"}))
    .route("/register", post(register_user))
    .route("/login", post(login_user))
    .route("/auth/{owner_id}/files/", post(create_file).route_layer(middleware::from_fn(auth_middleware)))
    .route("/auth/{owner_id}/files/{file_name}/", get(download_file).delete(delete_file).route_layer(middleware::from_fn(auth_middleware)))
    .route("/auth/{owner_id}/files/{file_name}/version/{version}/", get(download_file_by_version).delete(delete_file_by_version).route_layer(middleware::from_fn(auth_middleware)))
    .with_state(database_pool)
}

