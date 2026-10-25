//! In-memory fake for reading and writing application status.

use crate::prelude::*;

/// [`StatusProvider`] and [`StatusWriter`] that keep each [`Status`] in memory.
///
/// - Returns [`Status::fixture`] for an [`Id`] that was never set
#[derive(FromServices)]
pub struct StatusFake {
    #[di(default)]
    statuses: Mutex<HashMap<Id, Status>>,
}

impl StatusFake {
    /// Create a [`StatusFake`] fixture where [`Id::fixture`] is [`Status::Active`].
    pub fn fixture_active() -> Self {
        Self {
            statuses: Mutex::new(HashMap::from([(Id::fixture(), Status::Active)])),
        }
    }
}

impl StatusProvider for StatusFake {
    fn get(&self, id: &Id) -> Result<Status, Report<StatusProviderError>> {
        let statuses = self
            .statuses
            .lock()
            .expect("should be able to lock statuses");
        Ok(statuses.get(id).copied().unwrap_or_else(Status::fixture))
    }
}

impl StatusWriter for StatusFake {
    fn set(&self, id: &Id, status: Status) -> Result<(), Report<StatusWriterError>> {
        let mut statuses = self
            .statuses
            .lock()
            .expect("should be able to lock statuses");
        statuses.insert(id.clone(), status);
        Ok(())
    }
}
