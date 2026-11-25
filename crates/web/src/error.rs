//! Errors that can happen while handling a request. Handlers return
//! `Result<_, AppError>` and use `?` instead of `unwrap()` (PRD §12 rule 4).

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    Template(askama::Error),
}

impl From<askama::Error> for AppError {
    fn from(err: askama::Error) -> Self {
        AppError::Template(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Log the details for us; show the visitor a generic message only.
        match &self {
            AppError::Template(err) => tracing::error!(error = %err, "template render failed"),
        }
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html("<h1>Something went wrong</h1><p>Please try again later.</p>"),
        )
            .into_response()
    }
}
