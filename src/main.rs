use std::{error::Error, sync::Arc};

mod api;
use api::configure;
mod server_serve;
mod settings;
mod application_state;
mod services;
use crate::server_serve::server_serve;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = settings::Settings::new("config.json", "APP")?;
    let db_url = settings
    .database
    .url
    .clone()
    .expect("Database URL is not set");
    let pool = sqlx::MySqlPool::connect(&db_url).await?;
    let application_state = Arc::new(application_state::ApplicationState::new(&settings, pool)?);
    
    let app = configure(application_state.clone());
    server_serve(app, application_state).await?;
    Ok(())
}