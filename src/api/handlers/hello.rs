
use serde::Serialize;
use axum::Json;
use axum::extract::State;
use std::sync::Arc;

#[derive(Serialize)]
pub struct CustomMessage {
 message: String,
}

use crate::application_state::ApplicationState;

pub async fn test(State(state): State<Arc<ApplicationState>>) -> Json<CustomMessage> {

   let db_url = Arc::clone(&state).settings.load().database.url.clone().unwrap_or_else(|| "Database URL is missing".to_string());
   //let db_url = state.arc().settings.load().database.url.clone().unwrap_or_else(|| "Database URL is missing".to_string());
   // let db_url = state.settings.database.url.clone().unwrap_or_else(|| "Database URL is missing".to_string());
   let msg = CustomMessage {
      message: format!("Hello, World! Database URL: {}", db_url),
   };
   Json(msg)
}
