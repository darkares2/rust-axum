use axum::Json;
use axum::extract::State;
use std::sync::Arc;
use axum::extract::Path;
use crate::services::model::User;
use crate::services::user::UserService;
use crate::application_state::ApplicationState;
use serde::Serialize;
use crate::services::model::CreateUserRequest;
use axum_macros::debug_handler;

#[derive(Serialize)]
pub struct CustomMessage {
 message: String,
}

#[debug_handler]
pub async fn get_by_id(Path(user_id): Path<i64>, State(state): State<Arc<ApplicationState>>) -> Json<User> {

    let user_service = Arc::clone(&state).user_service.clone();
    let user = user_service.get_user_by_id(user_id).await.unwrap();
    Json(user)
}

pub async fn create(State(state): State<Arc<ApplicationState>>,
Json(body): Json<CreateUserRequest>) -> Json<User> {

    let user_service = Arc::clone(&state).user_service.clone();
    let user = user_service.create_user(body).await.unwrap();
    Json(user)
}

pub async fn delete(State(state): State<Arc<ApplicationState>>,
            Path(user_id): Path<i64>) -> Json<CustomMessage> {

    let user_service = Arc::clone(&state).user_service.clone();
    user_service.delete_user(user_id).await.unwrap();
    let custom_message = CustomMessage {
        message: format!("User with id {} deleted", user_id),
    };
    Json(custom_message)
}
