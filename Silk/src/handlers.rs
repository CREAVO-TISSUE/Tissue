use axum::{
    debug_handler,
    extract::{Form, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::SqlitePool;
use tracing::debug;

#[derive(serde::Deserialize, Debug)]
pub struct Subscription {
    pub email: String,
}

#[debug_handler]
pub async fn subscribe(
    State(state): State<SqlitePool>,
    Form(sub): Form<Subscription>,
) -> impl IntoResponse {
    let query = format!("INSERT INTO subscriptions (email) VALUES ('{}')", sub.email);
    if let Ok(_) = sqlx::query(&query).execute(&state).await {
        debug!("Subscribed {}", sub.email);
        StatusCode::OK
    } else {
        debug!("Failed to subscribe {}", sub.email);
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
