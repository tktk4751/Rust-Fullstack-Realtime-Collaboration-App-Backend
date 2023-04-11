use argon2::{self, Config};
use rand::Rng;
use std::sync::RwLock;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Password hashing error: {0}")]
    HashingError(String),
    #[error("Password verification error: {0}")]
    VerificationError(String),
}

pub fn hash_password(password: &str) -> Result<String, PasswordError> {
    let salt = RwLock::new(rand::thread_rng().gen::<[u8; 32]>());
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), &salt.read().unwrap(), &config)
        .map_err(|e| PasswordError::HashingError(e.to_string()))
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, PasswordError> {
    argon2::verify_encoded(hash, password.as_bytes())
        .map_err(|e| PasswordError::VerificationError(e.to_string()))
    }
