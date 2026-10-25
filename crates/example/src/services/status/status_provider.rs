//! Trait and stub for reading application status.

use crate::prelude::*;

/// Read the current application status.
pub trait StatusProvider: Send + Sync {
    /// Get the current status of `id`.
    fn get(&self, id: &Id) -> Result<Status, Report<StatusProviderError>>;
}

/// [`StatusProvider`] returning a fixed [`Status`] or [`StatusProviderError`].
#[cfg(test)]
pub struct StatusProviderStub {
    /// Value returned by [`StatusProvider::get`].
    pub status: Result<Status, StatusProviderError>,
}

#[cfg(test)]
impl FromServices for StatusProviderStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self {
            status: Ok(Status::fixture()),
        })
    }
}

#[cfg(test)]
impl StatusProvider for StatusProviderStub {
    fn get(&self, _id: &Id) -> Result<Status, Report<StatusProviderError>> {
        self.status.map_err(Report::new)
    }
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
