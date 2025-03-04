use std::error::Error;
use std::net::SocketAddr;

pub async fn server_serve(app: axum::Router) -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}