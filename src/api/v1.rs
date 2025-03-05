use axum::routing::get;
use axum::routing::post;
use axum::routing::delete;
use axum::Router;
use std::sync::Arc;

use super::handlers;

use crate::application_state::ApplicationState;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
     .route("/api/test", get(handlers::hello::test).with_state(state.clone()))
     .route("/api/user/{user_id}", get(handlers::user::get_by_id).with_state(state.clone()))
     .route("/api/user", post(handlers::user::create).with_state(state.clone()))
     .route("/api/user/{id}", delete(handlers::user::delete).with_state(state.clone()))
}