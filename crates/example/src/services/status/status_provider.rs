//! Trait for reading application status.

use crate::prelude::*;

/// Read the current application status.
pub trait StatusProvider: Send + Sync {
    /// Get the current status of `id`.
    fn get(&self, id: &Id) -> Result<Status, Report<StatusProviderError>>;
}

/// Errors returned by [`StatusProvider`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum StatusProviderError {
    /// Unable to read the status file.
    #[error("Unable to read status")]
    Read,
    /// The status file holds an unknown value.
    #[error("Unable to parse status")]
    Parse,
}
