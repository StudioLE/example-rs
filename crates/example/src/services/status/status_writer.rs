//! Trait for writing application status.

use crate::prelude::*;

/// Write the current application status.
pub trait StatusWriter: Send + Sync {
    /// Set the current status of `id`.
    fn set(&self, id: &Id, status: Status) -> Result<(), Report<StatusWriterError>>;
}

/// Errors returned by [`StatusWriter`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum StatusWriterError {
    /// Unable to write the status file.
    #[error("Unable to write status")]
    Write,
}
