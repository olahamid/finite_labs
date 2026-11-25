//! The Finite Labs web server as a library, so tests can build the exact same
//! `Router` that `main.rs` serves.

#![forbid(unsafe_code)]

pub mod error;
pub mod models;
pub mod routes;
mod security;

use std::{path::Path, sync::Arc};

use axum::{Router, routing::get};
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};

use models::SiteConfig;

/// Everything loaded once at startup and shared (read-only) by all requests.
pub struct AppState {
    pub site: SiteConfig,
}

pub fn app(state: Arc<AppState>, static_dir: impl AsRef<Path>) -> Router {
    let router = Router::new()
        .route("/", get(routes::home))
        .route("/about", get(routes::about))
        .route("/healthz", get(routes::healthz))
        .nest_service("/static", ServeDir::new(static_dir))
        .fallback(routes::not_found)
        .with_state(state);

    security::apply(router)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use axum::{
        body::Body,
        http::{HeaderMap, Request, StatusCode, header},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    /// Tests run from `crates/web`, so find the repo root from the manifest dir.
    fn repo_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    fn test_app() -> Router {
        let root = repo_root();
        let site = SiteConfig::load(&root.join("data/site.toml")).expect("site.toml should parse");
        app(Arc::new(AppState { site }), root.join("static"))
    }

    async fn get(uri: &str) -> (StatusCode, HeaderMap, String) {
        let response = test_app()
            .oneshot(
                Request::get(uri)
                    .body(Body::empty())
                    .expect("valid request"),
            )
            .await
            .expect("router is infallible");
        let status = response.status();
        let headers = response.headers().clone();
        let bytes = response
            .into_body()
            .collect()
            .await
            .expect("body should be readable")
            .to_bytes();
        (
            status,
            headers,
            String::from_utf8_lossy(&bytes).into_owned(),
        )
    }

    #[tokio::test]
    async fn home_page_renders_hero_sections_and_footer() {
        let (status, headers, body) = get("/").await;
        assert_eq!(status, StatusCode::OK);
        assert!(
            headers[header::CONTENT_TYPE]
                .to_str()
                .unwrap()
                .starts_with("text/html")
        );
        assert!(body.contains("put digital ownership and control back in your hands"));
        assert!(body.contains("site-footer"));
    }

    #[tokio::test]
    async fn playground_lives_inside_learn() {
        let (_, _, body) = get("/").await;
        let learn = body.find(r#"<section class="section" id="teach""#).unwrap();
        let playground = body
            .find(r#"<div class="subsection" id="playground""#)
            .expect("playground subsection");
        let next_section = body[learn + 1..]
            .find("<section ")
            .map(|i| i + learn + 1)
            .unwrap();
        assert!(learn < playground && playground < next_section);
        assert!(!body.contains(r#"<section class="section" id="playground""#));
        // Not a separate top-level nav item (the footer may still link to it).
        let nav_start = body.find(r#"<nav class="nav-desktop""#).unwrap();
        let nav = &body[nav_start..nav_start + body[nav_start..].find("</nav>").unwrap()];
        assert!(nav.contains(">Learn<"));
        assert!(!nav.contains(">Playground<"));
    }

    #[tokio::test]
    async fn education_follows_products() {
        // PRD §5.1: products lead slightly, education comes straight after.
        let (_, _, body) = get("/").await;
        let order = [
            "problem",
            "products",
            "teach",
            "research",
            "pillars",
            "audience",
            "early-access",
        ];
        let positions: Vec<usize> = order
            .iter()
            .map(|id| {
                body.find(&format!(r#"<section class="section" id="{id}""#))
                    .unwrap_or_else(|| panic!("missing section #{id}"))
            })
            .collect();
        assert!(
            positions.windows(2).all(|w| w[0] < w[1]),
            "sections out of order"
        );
    }

    #[tokio::test]
    async fn audience_section_has_five_cards() {
        let (_, _, body) = get("/").await;
        assert!(body.contains(r#"<ul class="cards cards-5">"#));
    }

    #[tokio::test]
    async fn live_proof_widget_ships_with_real_default_hash() {
        let (_, _, body) = get("/").await;
        assert!(body.contains(r#"id="live-proof""#));
        assert!(body.contains("For learning only"));
        let expected = crypto::hash::sha256_hex(routes::DEMO_MESSAGE.as_bytes());
        assert!(
            body.contains(&expected),
            "server-rendered SHA-256 of the default message"
        );
        assert!(body.contains(r#"<script type="module" src="/static/playground.js">"#));
    }

    #[tokio::test]
    async fn card_hex_is_real_sha256_prefix() {
        let (_, _, body) = get("/").await;
        let hex = &crypto::hash::sha256_hex(b"Researchers")[..8];
        assert!(body.contains(&format!(
            r#"<span class="card-hex" aria-hidden="true">{hex}</span>"#
        )));
        assert!(!body.contains(">0x"), "hex must not use a 0x prefix");
    }

    #[tokio::test]
    async fn about_page_renders() {
        let (status, _, body) = get("/about").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("About Finite Labs"));
    }

    #[tokio::test]
    async fn unknown_page_is_a_styled_404() {
        let (status, _, body) = get("/does-not-exist").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(body.contains("Page not found"));
    }

    #[tokio::test]
    async fn healthz_returns_ok() {
        let (status, _, body) = get("/healthz").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "ok");
    }

    #[tokio::test]
    async fn static_files_are_served() {
        for path in [
            "/static/style.css",
            "/static/theme.js",
            "/static/favicon.svg",
            "/static/playground.js",
            "/static/fonts/jetbrains-mono-latin-wght-normal.woff2",
        ] {
            let (status, _, _) = get(path).await;
            assert_eq!(status, StatusCode::OK, "{path}");
        }
    }

    #[tokio::test]
    async fn every_response_has_security_headers() {
        for path in ["/", "/about", "/nope", "/static/style.css"] {
            let (_, headers, _) = get(path).await;
            assert_eq!(
                headers[header::CONTENT_SECURITY_POLICY],
                security::CSP,
                "{path}"
            );
            assert!(
                headers.contains_key(header::STRICT_TRANSPORT_SECURITY),
                "{path}"
            );
            assert_eq!(headers[header::X_CONTENT_TYPE_OPTIONS], "nosniff", "{path}");
            assert_eq!(headers[header::X_FRAME_OPTIONS], "DENY", "{path}");
        }
    }

    #[tokio::test]
    async fn pages_have_no_inline_scripts_or_styles() {
        // The CSP blocks inline code, so any inline script/style would silently break.
        for path in ["/", "/about", "/nope"] {
            let (_, _, body) = get(path).await;
            assert!(!body.contains("<script>"), "{path} has an inline script");
            assert!(!body.contains("<style"), "{path} has an inline style block");
            assert!(
                !body.contains("style=\""),
                "{path} has an inline style attribute"
            );
        }
    }

    #[test]
    fn site_config_rejects_unknown_fields() {
        let raw = std::fs::read_to_string(repo_root().join("data/site.toml")).unwrap();
        let with_typo = format!("nmae = \"oops\"\n{raw}");
        assert!(toml::from_str::<SiteConfig>(&with_typo).is_err());
    }
}
