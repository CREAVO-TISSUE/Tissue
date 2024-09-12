use crate::templates::{IndexTemplate, PricingTemplate};
use askama::Template;
use axum::{
    debug_handler,
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};

mod subscribe;

pub use subscribe::subscribe;

static THEME_CSS: &str = include_str!("../assets/theme.css");

pub async fn assets(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    match path.as_str() {
        "theme.css" => {
            headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
            (StatusCode::OK, headers, THEME_CSS)
        }
        _ => (StatusCode::NOT_FOUND, headers, ""),
    }
}

pub async fn index() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "text/html; charset=utf-8".parse().unwrap(),
    );
    let template = IndexTemplate {};
    (StatusCode::OK, headers, template.render().unwrap())
}

#[debug_handler]
pub async fn pricing() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "text/html; charset=utf-8".parse().unwrap(),
    );
    //let template = PricingTemplate {};
    //(StatusCode::OK, headers, template.render().unwrap())
    let template = PricingTemplate {};
    template
}
