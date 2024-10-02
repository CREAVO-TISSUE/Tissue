use axum::{
    response::IntoResponse,
    routing::get,
    Router,
};
use anyhow::Result;
use tracing::info;

// Create a axum handler with expose a Tokio server with a single route
// The route should be a GET request to /fx/{amount}/{from}/
async fn handler() -> impl IntoResponse {
    String::from("hello world")
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // Create a new router
    let app = Router::new().route("/fx", get(handler));

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app)
        .await?;
    Ok(())
}
