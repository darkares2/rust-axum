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
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;

#[derive(Serialize)]
pub struct CustomMessage {
 message: String,
}


#[debug_handler]
pub async fn get_all(State(state): State<Arc<ApplicationState>>) -> Response {
    let user_service = Arc::clone(&state).user_service.clone();
    match user_service.get_all().await {
        Ok(users) => {
            return Json(users).into_response();
        },
        Err(err) => {
            eprintln!("Error fetching users: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    }
}
#[debug_handler]
pub async fn get_by_id(Path(user_id): Path<i64>, State(state): State<Arc<ApplicationState>>) -> Response {

    let user_service = Arc::clone(&state).user_service.clone();
    match user_service.get_user_by_id(user_id).await {
        Ok(user) => {
            return Json(user).into_response();
        },
        Err(err) => {            
            match err.downcast_ref::<sqlx::Error>() {
                Some(sqlx::Error::RowNotFound) => {
                    return (StatusCode::NOT_FOUND, err.to_string()).into_response();
                },
                _ => {}
            }
            eprintln!("Error fetching user: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    }
}

pub async fn create(State(state): State<Arc<ApplicationState>>,
Json(body): Json<CreateUserRequest>) -> Response {

    let user_service = Arc::clone(&state).user_service.clone();
    match user_service.create_user(body).await {
        Ok(user) => {
            return Json(user).into_response();
        },
        Err(err) => {
            eprintln!("Error creating user: {:?}", err);
            match err.downcast_ref::<sqlx::Error>() {
                Some(sqlx::Error::RowNotFound) => {
                    return (StatusCode::NOT_FOUND, err.to_string()).into_response();
                },
                _ => {}
            }
            eprintln!("Error fetching user: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    }
}

pub async fn delete(State(state): State<Arc<ApplicationState>>,
            Path(user_id): Path<i64>) -> Response {

    let user_service = Arc::clone(&state).user_service.clone();
    match user_service.delete_user(user_id).await {
        Ok(_) => {
            let custom_message = CustomMessage {
                message: format!("User with id {} deleted", user_id),
            };
            return Json(custom_message).into_response();
        },
        Err(err) => {
            eprintln!("Error deleting user: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    }
}
