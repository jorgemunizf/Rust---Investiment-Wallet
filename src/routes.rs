use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::home::home;

pub fn create_routes(pool: PgPool) -> Router {
    Router::new().route("/", get(home)).with_state(pool)
}
