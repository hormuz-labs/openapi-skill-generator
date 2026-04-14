use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum AppError {
    #[error("Failed to parse schema: {0}")]
    Parse(String),

    #[error("Failed to generate markdown: {0}")]
    Generate(String),

    #[error("Failed to create archive: {0}")]
    Archive(String),

    #[error("Multipart form error: {0}")]
    Multipart(String),

    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Parse(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Generate(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Archive(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Multipart(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
