use axum::{
    response::IntoResponse,
    routing::get,
    Router,
};
use anyhow::Result;

// Create a axum handler with expose a Tokio server with a single route
// The route should be a GET request to /fx/{amount}/{from}/
async fn handler(
) -> impl IntoResponse {
    String::from("hello world")
}

#[tokio::main]
async fn main() -> Result<()> {
    // Create a new router
    let app = Router::new().route("/fx", get(handler));

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app)
        .await?;
    Ok(())
}
