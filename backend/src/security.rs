use crate::errors::{AppError, AppResult};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use tokio::task::spawn_blocking;
use tracing::error;

pub async fn hash_password(password: String) -> AppResult<String> {
    spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| {
                error!("Failed to hash password: {}", e);
                AppError::InternalServerError
            })?
            .to_string();

        Ok(password_hash)
    })
    .await
    .map_err(|e| {
        error!("Task join error during password hash: {}", e);
        AppError::InternalServerError
    })?
}

pub async fn verify_password(hash: String, password: String) -> AppResult<bool> {
    spawn_blocking(move || {
        let parsed_hash = PasswordHash::new(&hash).map_err(|e| {
            error!("Failed to parse password hash: {}", e);
            AppError::InternalServerError
        })?;

        let argon2 = Argon2::default();

        let result = argon2.verify_password(password.as_bytes(), &parsed_hash);

        match result {
            Ok(_) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => {
                error!("Password verification error: {}", e);
                Err(AppError::InternalServerError)
            }
        }
    })
    .await
    .map_err(|e| {
        error!("Task join error during password verify: {}", e);
        AppError::InternalServerError
    })?
}
