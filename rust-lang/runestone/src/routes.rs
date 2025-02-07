use axum::{routing::get, Router};
use super::model::{get_all_runestones, create_runestone, get_runestone, update_runestone, delete_runestone};


pub fn all_route(database_pool: sqlx::Pool<sqlx::Postgres> ) -> Router {
    Router::new()
    .route("/", get(|| async {"Hello Aldino!"}))
    .route("/runestones", get(get_all_runestones).post(create_runestone))
    .route("/runestones/{runestone_id}", get(get_runestone).patch(update_runestone).delete(delete_runestone))
    .with_state(database_pool)
}

