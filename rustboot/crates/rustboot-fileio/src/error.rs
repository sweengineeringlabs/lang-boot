//! File I/O error types

use thiserror::Error;

pub type FileIoResult<T> = Result<T, FileIoError>;

#[derive(Debug, Error)]
pub enum FileIoError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Path error: {0}")]
    PathError(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

impl From<tempfile::PersistError> for FileIoError {
    fn from(err: tempfile::PersistError) -> Self {
        FileIoError::Io(err.error)
    }
}
