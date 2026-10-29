use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum EngineError {
    InvalidOperation(String),
    ResourceNotFound(String),
    InvalidData(String),
    IoError(String),
    OutOfMemory,
    UnsupportedFeature(String),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            EngineError::ResourceNotFound(msg) => write!(f, "Resource not found: {}", msg),
            EngineError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            EngineError::IoError(msg) => write!(f, "I/O error: {}", msg),
            EngineError::OutOfMemory => write!(f, "Out of memory"),
            EngineError::UnsupportedFeature(msg) => write!(f, "Unsupported feature: {}", msg),
        }
    }
}

impl std::error::Error for EngineError {}

pub type Result<T> = std::result::Result<T, EngineError>;

impl From<std::io::Error> for EngineError {
    fn from(err: std::io::Error) -> Self {
        EngineError::IoError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = EngineError::InvalidOperation("test".to_string());
        assert_eq!(err.to_string(), "Invalid operation: test");
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = EngineError::from(io_err);
        assert!(matches!(err, EngineError::IoError(_)));
    }
}
