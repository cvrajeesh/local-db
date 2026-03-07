use thiserror::Error;

/// All errors that can be produced by this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// An error from the underlying SQLite engine.
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// An error from the SQL translation layer.
    #[error("Translation error: {0}")]
    Translation(String),

    /// A type conversion error when reading a value from a row.
    #[error("Type conversion error: expected {expected}, got {actual}")]
    TypeConversion {
        expected: &'static str,
        actual: String,
    },

    /// Column index out of range.
    #[error("Column index {index} out of range (row has {count} columns)")]
    ColumnIndexOutOfRange { index: usize, count: usize },

    /// Named column not found in result set.
    #[error("Column '{name}' not found in result set")]
    ColumnNotFound { name: String },

    /// A JSON serialisation / deserialisation error (used for VARIANT columns).
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Generic I/O error (e.g. when opening a file-backed database).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Any other error not covered above.
    #[error("{0}")]
    Other(String),
}

impl Error {
    pub fn translation(msg: impl Into<String>) -> Self {
        Error::Translation(msg.into())
    }

    pub fn other(msg: impl Into<String>) -> Self {
        Error::Other(msg.into())
    }
}
