//! Request parameters for the stop subcommand.

use crate::prelude::*;

/// Parsed arguments for the stop subcommand.
#[derive(Args, Clone, Debug, Eq, PartialEq)]
pub struct StopRequest {
    /// Identifier of the resource to stop.
    pub id: Id,
}

impl StopRequest {
    /// Create a new [`StopRequest`] from an [`Id`].
    #[cfg_attr(not(test), expect(dead_code, reason = "example"))]
    pub fn new(id: Id) -> Self {
        Self { id }
    }

    /// Create a mock [`StopRequest`] for testing.
    #[cfg(test)]
    pub fn mock() -> Self {
        Self::new(Id::mock())
    }
}
