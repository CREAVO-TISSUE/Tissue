use askama_axum::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "pricing.html")]
pub struct PricingTemplate {}
