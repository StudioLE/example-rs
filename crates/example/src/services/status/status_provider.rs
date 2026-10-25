//! Trait and adapter for querying application status.

use crate::prelude::*;

/// Query the current application status.
pub trait StatusProvider: Send + Sync {
    /// Get the current status.
    fn get(&self, id: &Id) -> Result<Status, Report<StatusError>>;
}

/// Default [`StatusProvider`] implementation.
#[derive(Default, FromServices)]
pub struct StatusProviderAdapter;

impl StatusProvider for StatusProviderAdapter {
    fn get(&self, _id: &Id) -> Result<Status, Report<StatusError>> {
        Ok(Status::Inactive)
    }
}

/// [`StatusProvider`] returning a fixed [`Status`].
#[cfg(test)]
pub struct StatusProviderStub {
    /// Status returned by [`StatusProvider::get`].
    pub status: Status,
}

#[cfg(test)]
impl FromServices for StatusProviderStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self {
            status: Status::fixture(),
        })
    }
}

#[cfg(test)]
impl StatusProvider for StatusProviderStub {
    fn get(&self, _id: &Id) -> Result<Status, Report<StatusError>> {
        Ok(self.status)
    }
}

/// Errors returned by [`StatusProvider`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum StatusError {
    /// Unable to read status.
    #[expect(dead_code, reason = "example")]
    #[error("Failed to read status")]
    Read,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [`StatusProviderAdapter`] returns idle by default.
    #[test]
    fn status_provider_adapter_get() {
        // Arrange
        let status = StatusProviderAdapter;
        let id = Id::fixture();
        // Act
        let result = status.get(&id);
        // Assert
        let status = result.expect("should be able to get status");
        assert_eq!(status, Status::Inactive);
    }
}
