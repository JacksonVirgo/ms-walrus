pub mod homepage;

use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new()
        .route("/", get(homepage::index))
        .route("/clicked", get(homepage::clicked))
}
