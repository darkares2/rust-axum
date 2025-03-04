use axum::routing::get;
use axum::Router;
use std::sync::Arc;

use super::handlers;

use crate::settings::Settings;

pub fn configure(settings: Arc<Settings>) -> Router {
    Router::new()
     .route("/api/test", get(handlers::hello::test).with_state(settings))
}