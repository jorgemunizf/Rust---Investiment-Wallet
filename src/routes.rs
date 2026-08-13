use axum::{Router, routing::get};

use crate::handlers::home::home;

pub fn create_routes() -> Router {
    Router::new().route("/", get(home))
}
