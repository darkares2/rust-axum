use std::{error::Error, sync::Arc};

mod api;
use api::configure;
mod server_serve;
mod settings;
use crate::server_serve::server_serve;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = settings::Settings::new("config.json", "APP")?;
    let shared_settings = Arc::new(settings);
    let app = configure(shared_settings.clone());
    server_serve(app, shared_settings).await?;
    Ok(())
}