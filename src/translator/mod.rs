//! SQL translation layer — Snowflake dialect → SQLite dialect.
//!
//! The translation is performed in several passes:
//!
//! 1. **No-op detection** – statements that don't map to SQLite (e.g. `USE DATABASE`,
//!    `ALTER SESSION`) are recognised and replaced with a benign SQL comment.
//! 2. **DDL rewriting** – `CREATE OR REPLACE TABLE` becomes `DROP + CREATE`, Snowflake
//!    type names are mapped to SQLite affinities, column-level options that don't exist
//!    in SQLite (e.g. `AUTOINCREMENT` as Snowflake uses it) are normalised.
//! 3. **Identifier stripping** – fully-qualified `db.schema.table` identifiers are
//!    reduced to just `table` (optionally `schema_table` to avoid collisions).
//! 4. **Function rewriting** – Snowflake scalar functions are converted to their SQLite
//!    equivalents (see [`functions`]).
//! 5. **Operator / syntax rewriting** – `ILIKE`, semi-structured path expressions, etc.

pub mod functions;
pub mod identifiers;
pub mod noop;
pub mod rewriter;
pub mod types;

pub use rewriter::Translator;

use crate::Result;

/// Translate a Snowflake SQL statement into an equivalent SQLite statement.
///
/// Returns `None` if the statement is a known no-op (e.g. `USE DATABASE foo`).
pub fn translate(sql: &str) -> Result<Option<String>> {
    Translator::default().translate(sql)
}

/// Translate a batch of semicolon-separated Snowflake SQL statements.
///
/// No-op statements are silently dropped from the output.
pub fn translate_batch(sql: &str) -> Result<Vec<String>> {
    Translator::default().translate_batch(sql)
}
