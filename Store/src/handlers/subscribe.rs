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
    let res = reqwest::get("http://127.0.0.1:4000/subscribe").await;
    if res.unwrap().status().is_success() {
        info!("Subscribed: {}", subscribe.email);
    } else {
        info!("Failed to subscribe: {}", subscribe.email);
    }
    Redirect::to("/?subscribed=true").into_response()
}
