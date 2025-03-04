
use serde::Serialize;
use axum::Json;
use axum::extract::State;
use std::sync::Arc;

#[derive(Serialize)]
pub struct CustomMessage {
 message: String,
}

use crate::settings::Settings;

pub async fn test(State(settings): State<Arc<Settings>>) -> Json<CustomMessage> {
    let db_url = settings.database.url.clone().unwrap_or_else(|| "Database URL is missing".to_string());
 let msg = CustomMessage {
    message: format!("Hello, World! Database URL: {}", db_url),
 };
 Json(msg)
}
