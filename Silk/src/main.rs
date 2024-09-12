use anyhow::Result; // Use the Result type from the anyhow crate
use axum::{routing::post, Router};
use tower_http::trace::TraceLayer;
use tracing::debug; // Use the Router and get types from the axum crate

mod handlers;

use handlers::subscribe; // Import the subscribe module

const ADDR: &str = "127.0.0.1:4000"; // Address to bind to

#[tokio::main] // Main function
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init(); // Initialize the logger

    let app = Router::new() // Create a new Router
        .route("/subscribe", post(subscribe)) // Add a route to the Router
        .layer(TraceLayer::new_for_http()); // Add a tracing layer to the Router.

    let listener = tokio::net::TcpListener::bind(ADDR).await?; // Bind to the address
    debug!("Listening on {}", ADDR); // Log that we are listening

    axum::serve(listener, app.into_make_service()).await?; // Serve the app
    Ok(())
}
