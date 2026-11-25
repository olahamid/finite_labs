#![forbid(unsafe_code)]

use std::{path::Path, sync::Arc};

use web::{AppState, models::SiteConfig};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Startup is the one place where panicking is fine: bad config should stop
    // the server immediately, with a clear message (PRD §12 rule 4).
    let site = SiteConfig::load(Path::new("data/site.toml")).unwrap_or_else(|err| {
        panic!("{err} (run the server from the repository root)");
    });

    let app = web::app(Arc::new(AppState { site }), "static");

    // Render sets PORT; default to 3000 locally.
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|err| panic!("failed to bind {addr}: {err}"));
    tracing::info!("listening on http://localhost:{port}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
}

/// Wait for Ctrl+C locally, or SIGTERM from Docker/Render, then let in-flight
/// requests finish before exiting.
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen for Ctrl+C");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to listen for SIGTERM")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
    tracing::info!("shutting down");
}
