//! Trait and spy for writing application status.

use crate::prelude::*;

/// Write the current application status.
pub trait StatusWriter: Send + Sync {
    /// Set the current status of `id`.
    fn set(&self, id: &Id, status: Status) -> Result<(), Report<StatusWriterError>>;
}

/// [`StatusWriter`] that records each call and always succeeds.
#[cfg(test)]
#[derive(FromServices)]
pub struct StatusWriterSpy {
    #[di(default)]
    calls: Mutex<Vec<StatusWriterCall>>,
}

/// Arguments of one [`StatusWriter::set`] call recorded by [`StatusWriterSpy`].
#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusWriterCall {
    /// Identifier passed to [`StatusWriter::set`].
    pub id: Id,
    /// Status passed to [`StatusWriter::set`].
    pub status: Status,
}

#[cfg(test)]
impl StatusWriterSpy {
    /// Each [`StatusWriter::set`] call, in call order.
    pub fn calls(&self) -> Vec<StatusWriterCall> {
        self.calls
            .lock()
            .expect("should be able to lock calls")
            .clone()
    }
}

#[cfg(test)]
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

/// Errors returned by [`StatusWriter`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum StatusWriterError {
    /// Unable to write the status file.
    #[error("Unable to write status")]
    Write,
}
