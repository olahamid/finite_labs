//! Page handlers. Each page is an Askama template struct; Askama checks the
//! template against the struct at compile time, so a typo in a template is a
//! build error, not a runtime crash.

use std::sync::Arc;

use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use chrono::Datelike;

use crate::{AppState, error::AppError, models::SiteConfig};

fn current_year() -> i32 {
    chrono::Utc::now().year()
}

fn render(template: &impl Template) -> Result<Html<String>, AppError> {
    Ok(Html(template.render()?))
}

/// The Live Proof widget's starting text. Tampering turns ₦5,000 into ₦9,000.
pub const DEMO_MESSAGE: &str = "Send ₦5,000 to Ada";

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate<'a> {
    site: &'a SiteConfig,
    year: i32,
    demo_message: &'static str,
    /// Real SHA-256 of `demo_message`, so the widget shows a true value even
    /// before (or without) WebAssembly.
    demo_hash: String,
}

pub async fn home(State(state): State<Arc<AppState>>) -> Result<Html<String>, AppError> {
    render(&HomeTemplate {
        site: &state.site,
        year: current_year(),
        demo_message: DEMO_MESSAGE,
        demo_hash: crypto::hash::sha256_hex(DEMO_MESSAGE.as_bytes()),
    })
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate<'a> {
    site: &'a SiteConfig,
    year: i32,
}

pub async fn about(State(state): State<Arc<AppState>>) -> Result<Html<String>, AppError> {
    render(&AboutTemplate {
        site: &state.site,
        year: current_year(),
    })
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate<'a> {
    site: &'a SiteConfig,
    year: i32,
}

pub async fn not_found(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    let page = render(&NotFoundTemplate {
        site: &state.site,
        year: current_year(),
    })?;
    Ok((StatusCode::NOT_FOUND, page))
}

pub async fn healthz() -> &'static str {
    "ok"
}
