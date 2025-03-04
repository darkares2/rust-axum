
use serde::Serialize;
use axum::Json;

#[derive(Serialize)]
pub struct CustomMessage {
 message: String,
}

pub async fn test() -> Json<CustomMessage> {
 let msg = CustomMessage {
     message: "Hello, World!".to_string(),
 };
 Json(msg)
}
