use std::error::Error;
use std::fmt;

/// Custom error type for the AI Cache Cleaner CLI application.
#[derive(Debug)]
pub enum CleanerError {
    /// Wrapper for standard input/output errors.
    Io(std::io::Error),
    /// Wrapper for JSON parsing/serialization errors.
    Json(serde_json::Error),
    /// Wrapper for interactive prompt errors from the `dialoguer` crate.
    Prompt(dialoguer::Error),
    /// Custom error message.
    #[allow(dead_code)]
    Custom(String),
}

impl fmt::Display for CleanerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CleanerError::Io(err) => write!(f, "IO error: {}", err),
            CleanerError::Json(err) => write!(f, "JSON error: {}", err),
            CleanerError::Prompt(err) => write!(f, "Prompt error: {}", err),
            CleanerError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for CleanerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CleanerError::Io(err) => Some(err),
            CleanerError::Json(err) => Some(err),
            CleanerError::Prompt(err) => Some(err),
            CleanerError::Custom(_) => None,
        }
    }
}

impl From<std::io::Error> for CleanerError {
    fn from(err: std::io::Error) -> Self {
        CleanerError::Io(err)
    }
}

impl From<serde_json::Error> for CleanerError {
    fn from(err: serde_json::Error) -> Self {
        CleanerError::Json(err)
    }
}

impl From<dialoguer::Error> for CleanerError {
    fn from(err: dialoguer::Error) -> Self {
        CleanerError::Prompt(err)
    }
}

/// Type alias for standard Results in the cleaner module.
pub type Result<T> = std::result::Result<T, CleanerError>;
