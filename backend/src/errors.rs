use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

// --- 1. NEW: Import sqlx::Error ---
use sqlx::Error as SqlxError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("An internal server error occurred")]
    InternalServerError,

    #[error("Validation failed")]
    ValidationError(#[from] ValidationErrors),

    // --- 2. NEW: Add a variant for database errors ---
    #[error("Database error")]
    DatabaseError(#[from] SqlxError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Error: {:?}", self);

        let (status, error_message) = match self {
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"error": "Internal Server Error"}),
            ),

            AppError::ValidationError(errors) => {
                (StatusCode::BAD_REQUEST, json!({"errors": errors}))
            }

            // --- 3. NEW: Handle database errors ---
            AppError::DatabaseError(err) => {
                // Check if this is a "unique constraint" violation
                if let Some(db_err) = err.as_database_error() {
                    if db_err.is_unique_violation() {
                        // This likely means the email is already taken
                        return (
                            StatusCode::CONFLICT, // 409 Conflict
                            Json(json!({"error": "Email already in use."})),
                        )
                            .into_response();
                    }
                }

                // For all other database errors, return a generic 500
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({"error": "Internal Server Error"}),
                )
            }
        };

        (status, Json(error_message)).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
