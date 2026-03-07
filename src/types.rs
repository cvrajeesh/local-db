use serde::{Deserialize, Serialize};
use std::fmt;

/// A dynamically-typed SQL value that can be bound as a query parameter
/// or returned as a column value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    /// SQL NULL
    Null,
    /// Integer value (maps to Snowflake NUMBER with scale 0)
    Integer(i64),
    /// Floating-point value (maps to Snowflake FLOAT / NUMBER with scale > 0)
    Real(f64),
    /// Text value (maps to VARCHAR, CHAR, STRING, TEXT, VARIANT, etc.)
    Text(String),
    /// Raw bytes (maps to BINARY / VARBINARY)
    Blob(Vec<u8>),
    /// Boolean (stored as 0/1 in SQLite)
    Boolean(bool),
}

impl Value {
    /// Returns `true` if the value is `NULL`.
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Returns a human-readable type name for error messages.
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Null => "NULL",
            Value::Integer(_) => "INTEGER",
            Value::Real(_) => "REAL",
            Value::Text(_) => "TEXT",
            Value::Blob(_) => "BLOB",
            Value::Boolean(_) => "BOOLEAN",
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "NULL"),
            Value::Integer(i) => write!(f, "{i}"),
            Value::Real(r) => write!(f, "{r}"),
            Value::Text(s) => write!(f, "{s}"),
            Value::Blob(b) => write!(f, "<{} bytes>", b.len()),
            Value::Boolean(b) => write!(f, "{b}"),
        }
    }
}

// ── From impls ──────────────────────────────────────────────────────────────

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Integer(v)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Integer(v as i64)
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Value::Integer(v as i64)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Real(v)
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Value::Real(v as f64)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::Text(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::Text(v.to_owned())
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Boolean(v)
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Value::Blob(v)
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(v) => v.into(),
            None => Value::Null,
        }
    }
}

// ── rusqlite ToSql / FromSql ─────────────────────────────────────────────────

impl rusqlite::types::ToSql for Value {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        use rusqlite::types::{ToSqlOutput, ValueRef};
        Ok(match self {
            Value::Null => ToSqlOutput::Owned(rusqlite::types::Value::Null),
            Value::Integer(i) => ToSqlOutput::Owned(rusqlite::types::Value::Integer(*i)),
            Value::Real(r) => ToSqlOutput::Owned(rusqlite::types::Value::Real(*r)),
            Value::Text(s) => ToSqlOutput::Borrowed(ValueRef::Text(s.as_bytes())),
            Value::Blob(b) => ToSqlOutput::Borrowed(ValueRef::Blob(b)),
            Value::Boolean(b) => {
                ToSqlOutput::Owned(rusqlite::types::Value::Integer(if *b { 1 } else { 0 }))
            }
        })
    }
}
