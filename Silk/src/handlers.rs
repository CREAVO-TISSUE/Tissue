use axum::{debug_handler, http::StatusCode, response::IntoResponse};
use axum_extra::extract::Form;
use tracing::debug;

#[derive(serde::Deserialize, Debug, Default)]
pub struct Subscription {
    pub email: String,
}

#[debug_handler]
pub async fn subscribe(Form(sub): Form<Subscription>) -> impl IntoResponse {
    debug!("Subscribed with email: {}", sub.email);
    StatusCode::OK
}
