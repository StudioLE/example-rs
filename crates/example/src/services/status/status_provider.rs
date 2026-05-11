//! Trait and adapter for querying application status.

use crate::prelude::*;

/// Query the current application status.
#[cfg_attr(test, mockall::automock)]
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
        let id = Id::mock();
        // Act
        let result = status.get(&id);
        // Assert
        let status = result.expect("should be able to get status");
        assert_eq!(status, Status::Inactive);
    }

    /// [`MockStatusProvider`] returns the configured value.
    #[test]
    fn status_provider_mock_get() {
        // Arrange
        let mut status = MockStatusProvider::new();
        status.expect_get().returning(|_id| Ok(Status::Active));
        let id = Id::mock();
        // Act
        let result = status.get(&id);
        // Assert
        let status = result.expect("should be able to get status");
        assert_eq!(status, Status::Active);
        assert_yaml_snapshot!(status)
    }
}
