use axum::{debug_handler, http::StatusCode, response::IntoResponse, extract::Form};
use tracing::debug;

#[derive(serde::Deserialize, Debug)]
pub struct Subscription {
    pub email: String,
}

#[debug_handler]
pub async fn subscribe(Form(sub): Form<Subscription>) -> impl IntoResponse {
    debug!("Subscribed with email: {}", sub.email);
    StatusCode::OK
}
