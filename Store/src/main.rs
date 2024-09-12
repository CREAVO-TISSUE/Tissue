use anyhow::Result;
use axum::{routing::{get, post}, Router};
use handlers::{assets, index, pricing, subscribe};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

mod handlers;
mod templates;

const ADDR: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index))
        .route("/pricing", get(pricing))
        .route("/subscribe", post(subscribe))
        .route("/_assets/*path", get(assets))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(ADDR).await?;

    tracing::info!("listening on {}", ADDR);

    Ok(axum::serve(listener, app.into_make_service()).await?)
}
