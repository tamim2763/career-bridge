use crate::errors::AppResult;
use crate::models::{CareerTrack, ExperienceLevel};
use crate::security::hash_password;
use axum::{Router, routing::post};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(length(min = 1, message = "Full name is required"))]
    full_name: String,

    #[validate(email(message = "Invalid email format"))]
    email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    password: String,

    education_level: Option<String>,
    experience_level: ExperienceLevel,
    preferred_track: CareerTrack,
}

async fn register(axum::Json(payload): axum::Json<RegisterPayload>) -> AppResult<&'static str> {
    payload.validate()?;

    tracing::info!("New user registration (validated): {:?}", payload.email);

    let hashed_password = hash_password(payload.password).await?;

    tracing::debug!("Hashed password for user: {}", hashed_password);

    // TODO:
    // Save the user to the database

    Ok("User registered successfully")
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", axum::routing::get(root))
        .route("/error", axum::routing::get(test_error_handler))
        .route("/register", post(register))
}

async fn root() -> AppResult<&'static str> {
    tracing::info!("Request received for /");
    Ok("Hello, World!")
}

async fn test_error_handler() -> AppResult<&'static str> {
    tracing::info!("Request received for /error - this will fail!");
    Err(crate::errors::AppError::InternalServerError)
}
