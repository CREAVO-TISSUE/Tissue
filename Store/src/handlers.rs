use crate::templates::{IndexTemplate, PricingTemplate};
use askama::Template;
use axum::{
    body::Body,
    debug_handler,
    extract::Path,
    http::{header, HeaderMap, Response, StatusCode},
    response::IntoResponse,
};

mod subscribe;

pub use subscribe::subscribe;

static THEME_CSS: &str = include_str!("../assets/style/theme.css");
static INDEX_HEADER_IMAGE: &[u8] = include_bytes!("../assets/static/rocket_in_desert.webp");

#[axum::debug_handler]
pub async fn assets(Path(path): Path<String>) -> Response<Body> {
    let mut headers = HeaderMap::new();
    match path.as_str() {
        "theme.css" => {
            headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
            (StatusCode::OK, headers, THEME_CSS).into_response()
        }
        "rocket_in_desert.webp" => {
            headers.insert(header::CONTENT_TYPE, "image/webp".parse().unwrap());
            (StatusCode::OK, headers, INDEX_HEADER_IMAGE).into_response()
        }
        _ => (StatusCode::NOT_FOUND, headers, "").into_response(),
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
