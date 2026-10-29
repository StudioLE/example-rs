//! Trait for writing lines.

use crate::prelude::*;

/// Write lines of text.
pub trait WriteLine: Send + Sync {
    /// Write `line` followed by a newline.
    fn write_line(&self, line: &str) -> Result<(), Report<WriteLineError>>;
}

/// Errors returned by [`WriteLine`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum WriteLineError {
    /// Unable to write the line.
    #[error("Unable to write line")]
    Write,
}
