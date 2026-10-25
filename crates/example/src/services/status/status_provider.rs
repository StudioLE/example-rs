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

/// [`StatusProvider`] returning a fixed [`Status`] or [`StatusError`].
#[cfg(test)]
pub struct StatusProviderStub {
    /// Value returned by [`StatusProvider::get`].
    pub status: Result<Status, StatusError>,
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
    fn get(&self, _id: &Id) -> Result<Status, Report<StatusError>> {
        self.status.map_err(Report::new)
    }
}

/// [`StatusProvider`] returning a fixed [`Status`] and recording each [`Id`] it receives.
#[cfg(test)]
pub struct StatusProviderSpy {
    /// Status returned by [`StatusProvider::get`].
    pub status: Status,
    calls: Mutex<Vec<Id>>,
}

#[cfg(test)]
impl StatusProviderSpy {
    /// Each [`Id`] received by [`StatusProvider::get`], in call order.
    pub fn calls(&self) -> Vec<Id> {
        self.calls
            .lock()
            .expect("should be able to lock calls")
            .clone()
    }
}

#[cfg(test)]
impl FromServices for StatusProviderSpy {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self {
            status: Status::fixture(),
            calls: Mutex::default(),
        })
    }
}

#[cfg(test)]
impl StatusProvider for StatusProviderSpy {
    fn get(&self, id: &Id) -> Result<Status, Report<StatusError>> {
        self.calls
            .lock()
            .expect("should be able to lock calls")
            .push(id.clone());
        Ok(self.status)
    }
}

/// Errors returned by [`StatusProvider`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum StatusError {
    /// Unable to read status.
    #[cfg_attr(not(test), expect(dead_code, reason = "example"))]
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
