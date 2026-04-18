//! Error types for the NDJSON parser

use thiserror::Error;

/// Result type alias for parser operations
pub type Result<T> = std::result::Result<T, ParseError>;

/// Errors that can occur during parsing
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Error reading file from disk
    #[error("io error: {0}")]
    Io(String),

    /// Error parsing JSON
    #[error("json parse error at line {line}: {message}")]
    JsonParse { line: usize, message: String },

    /// Missing name reference
    #[error("missing name with id {id}")]
    MissingName { id: u32 },

    /// Missing level reference
    #[error("missing level with id {id}")]
    MissingLevel { id: u32 },

    /// Missing expression reference
    #[error("missing expression with id {id}")]
    MissingExpr { id: u32 },

    /// Invalid binder info string
    #[error("invalid binder info: {0}")]
    InvalidBinderInfo(String),

    /// Invalid safety string
    #[error("invalid safety: {0}")]
    InvalidSafety(String),

    /// Invalid hint type
    #[error("invalid hint: {0}")]
    InvalidHint(String),

    /// Invalid quot kind
    #[error("invalid quot kind: {0}")]
    InvalidQuotKind(String),

    /// Resolution error - forward reference could not be resolved
    #[error("unresolved forward reference: {kind} with id {id}")]
    UnresolvedForwardRef { kind: String, id: u32 },

    /// Cycle detected in resolution
    #[error("cycle detected in {kind} resolution at id {id}")]
    ResolutionCycle { kind: String, id: u32 },

    /// Unsupported export format version
    #[error("unsupported format version: {version}")]
    UnsupportedVersion { version: String },

    /// Missing metadata (required for some operations)
    #[error("missing metadata in export file")]
    MissingMetadata,
}

impl From<std::io::Error> for ParseError {
    fn from(e: std::io::Error) -> Self {
        ParseError::Io(e.to_string())
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(e: serde_json::Error) -> Self {
        ParseError::JsonParse {
            line: e.line(),
            message: e.to_string(),
        }
    }
}
