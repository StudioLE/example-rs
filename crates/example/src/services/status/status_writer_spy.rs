//! Spy for writing application status.

use crate::prelude::*;

/// [`StatusWriter`] that records each call and always succeeds.
#[derive(FromServices)]
pub struct StatusWriterSpy {
    #[di(default)]
    calls: Mutex<Vec<StatusWriterCall>>,
}

/// Arguments of one [`StatusWriter::set`] call recorded by [`StatusWriterSpy`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusWriterCall {
    /// Identifier passed to [`StatusWriter::set`].
    pub id: Id,
    /// Status passed to [`StatusWriter::set`].
    pub status: Status,
}

impl StatusWriterSpy {
    /// Each [`StatusWriter::set`] call, in call order.
    pub fn calls(&self) -> Vec<StatusWriterCall> {
        self.calls
            .lock()
            .expect("should be able to lock calls")
            .clone()
    }
}

impl StatusWriter for StatusWriterSpy {
    fn set(&self, id: &Id, status: Status) -> Result<(), Report<StatusWriterError>> {
        self.calls
            .lock()
            .expect("should be able to lock calls")
            .push(StatusWriterCall {
                id: id.clone(),
                status,
            });
        Ok(())
    }
}
