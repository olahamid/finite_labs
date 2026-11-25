//! Security headers added to every response (PRD §6.4 rule 3).

use axum::{
    Router,
    http::{HeaderName, HeaderValue, header},
};
use tower_http::set_header::SetResponseHeaderLayer;

/// No inline scripts or styles, no third-party origins. `'wasm-unsafe-eval'`
/// lets the M3 playground compile WebAssembly without allowing JS `eval`.
pub const CSP: &str = "default-src 'self'; \
    script-src 'self' 'wasm-unsafe-eval'; \
    style-src 'self'; \
    img-src 'self' data:; \
    font-src 'self'; \
    connect-src 'self'; \
    object-src 'none'; \
    base-uri 'none'; \
    form-action 'self'; \
    frame-ancestors 'none'";

pub fn apply(router: Router) -> Router {
    let headers: [(HeaderName, &'static str); 7] = [
        (header::CONTENT_SECURITY_POLICY, CSP),
        // Two years, as recommended for HSTS preload. Browsers ignore it over plain HTTP.
        (
            header::STRICT_TRANSPORT_SECURITY,
            "max-age=63072000; includeSubDomains",
        ),
        (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
        (header::REFERRER_POLICY, "strict-origin-when-cross-origin"),
        (header::X_FRAME_OPTIONS, "DENY"),
        (
            HeaderName::from_static("permissions-policy"),
            "camera=(), microphone=(), geolocation=()",
        ),
        (
            HeaderName::from_static("cross-origin-opener-policy"),
            "same-origin",
        ),
    ];

    headers.into_iter().fold(router, |router, (name, value)| {
        router.layer(SetResponseHeaderLayer::overriding(
            name,
            HeaderValue::from_static(value),
        ))
    })
}
