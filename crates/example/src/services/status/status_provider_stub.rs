//! Stub for reading application status.

use crate::prelude::*;

/// [`StatusProvider`] returning a fixed [`Status`] or [`StatusProviderError`].
pub struct StatusProviderStub {
    /// Value returned by [`StatusProvider::get`].
    pub status: Result<Status, StatusProviderError>,
}

impl FromServices for StatusProviderStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self {
            status: Ok(Status::fixture()),
        })
    }
}

impl StatusProvider for StatusProviderStub {
    fn get(&self, _id: &Id) -> Result<Status, Report<StatusProviderError>> {
        self.status.map_err(Report::new)
    }
}
