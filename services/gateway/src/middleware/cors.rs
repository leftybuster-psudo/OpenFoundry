use axum::http::{HeaderValue, Method};
use tower_http::cors::{AllowHeaders, AllowOrigin, CorsLayer};

pub fn cors_layer(origins: &[String]) -> CorsLayer {
    // Credentialed CORS cannot use wildcard allow-headers or allow-origin.
    // Mirror the request so local dev origins keep working without a wildcard.
    let layer = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(AllowHeaders::mirror_request())
        .allow_credentials(true)
        .max_age(std::time::Duration::from_secs(3600));

    if origins.is_empty() {
        layer.allow_origin(AllowOrigin::mirror_request())
    } else {
        let origins: Vec<HeaderValue> = origins.iter().filter_map(|o| o.parse().ok()).collect();
        layer.allow_origin(origins)
    }
}
