use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::application_state::ApplicationState;

pub async fn server_serve(app: axum::Router, state: Arc<ApplicationState>) -> Result<(), Box<dyn Error>> {
    let settings = state.settings.load();
    let address = settings.server.address.as_ref().ok_or("Server address is missing")?.parse::<std::net::Ipv4Addr>()?;
    let addr = SocketAddr::new(address.into(), *settings.server.port.as_ref().ok_or("Server port is missing")?);
    println!("Listening on: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}