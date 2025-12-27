//! Crypto error types

use thiserror::Error;

pub type CryptoResult<T> = Result<T, CryptoError>;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Hashing error: {0}")]
    HashError(String),

    #[error("Bcrypt error: {0}")]
    BcryptError(#[from] bcrypt::BcryptError),

    #[error("Verification failed")]
    VerificationFailed,
}
