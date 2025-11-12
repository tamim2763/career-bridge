//! Authentication handlers for user registration and login.

use axum::{extract::State, Json};
use validator::Validate;
use crate::models::{User, ExperienceLevel, CareerTrack};
use crate::errors::{AppResult, AppError};
use crate::security::{hash_password, verify_password};
use crate::auth::create_jwt;
use crate::AppState;
use super::types::{RegisterPayload, LoginPayload, LoginResponse, UserProfile};

/// Registers a new user account with simplified onboarding.
/// 
/// Creates a new user with only name, email, and hashed password.
/// The user can complete their profile later via the profile completion endpoint.
/// Returns a JWT token for immediate authentication.
/// 
/// # Errors
/// 
/// Returns an error if:
/// - Validation fails (invalid email, short password, etc.)
/// - Email is already registered (unique constraint violation)
/// - Database operation fails
/// - JWT token generation fails
pub async fn register(
    State(app_state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> AppResult<Json<serde_json::Value>> {
    payload.validate()?;
    
    tracing::info!("New user registration: {:?}", payload.email);

    let hashed_password = hash_password(payload.password).await?;
    
    let user_id = sqlx::query_scalar!(
        r#"
        INSERT INTO users (full_name, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        payload.full_name,
        payload.email,
        hashed_password
    )
    .fetch_one(&app_state.db_pool)
    .await?;

    // Generate JWT token for immediate login
    let token = create_jwt(user_id, payload.email.clone())?;

    Ok(Json(serde_json::json!({
        "message": "User registered successfully",
        "token": token,
        "user_id": user_id
    })))
}

/// Authenticates a user and returns a JWT token with profile information.
/// 
/// Verifies email and password, then generates a JWT token for authenticated requests.
/// Returns user profile including `profile_completed` status to determine if
/// onboarding is required.
/// 
/// # Errors
/// 
/// Returns an error if:
/// - Validation fails
/// - User with email doesn't exist
/// - Password doesn't match
/// - Token generation fails
pub async fn login(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> AppResult<Json<LoginResponse>> {
    payload.validate()?;
    
    tracing::info!("Login attempt for: {:?}", payload.email);

    // Fetch user from database
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT 
            id, full_name, email, education_level,
            experience_level as "experience_level: ExperienceLevel",
            preferred_track as "preferred_track: CareerTrack",
            profile_completed as "profile_completed!",
            skills, projects, target_roles, raw_cv_text, password_hash
        FROM users 
        WHERE email = $1
        "#,
        payload.email
    )
    .fetch_optional(&app_state.db_pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    // Verify password
    let is_valid = verify_password(user.password_hash.clone(), payload.password).await?;
    if !is_valid {
        return Err(AppError::Unauthorized);
    }

    // Generate JWT token
    let token = create_jwt(user.id, user.email.clone())?;

    Ok(Json(LoginResponse {
        token,
        user: UserProfile {
            id: user.id,
            full_name: user.full_name,
            email: user.email,
            profile_completed: user.profile_completed,
            education_level: user.education_level,
            experience_level: user.experience_level,
            preferred_track: user.preferred_track,
            skills: user.skills,
            projects: user.projects,
            target_roles: user.target_roles,
        },
    }))
}
