use axum::routing::get;
use axum::Router;

use super::handlers;

pub fn configure() -> Router {
    Router::new()
     .route("/api/test", get(handlers::hello::test).post(handlers::hello::test))
}