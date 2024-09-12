use axum::{
    extract::Form,
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
pub struct Subscribe {
    pub email: String,
}

pub async fn subscribe(Form(subscribe): Form<Subscribe>) -> impl IntoResponse {
    info!("Subscribed: {}", subscribe.email);
    Redirect::temporary("/")
}
