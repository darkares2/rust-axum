use std::sync::Arc;

use axum::Router;
use axum::routing::get_service;
use tower_http::services::ServeDir;

use crate::application_state::ApplicationState;

pub mod handlers;
mod v1;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .fallback_service(get_service(ServeDir::new("./site")))
        .nest("/v1", v1::configure(state))
}