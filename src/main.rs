use std::error::Error;

mod api;
use api::configure;
mod server_serve;
use crate::server_serve::server_serve;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = configure();
    server_serve(app).await?;
    Ok(())
}