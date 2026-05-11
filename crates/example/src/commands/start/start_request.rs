//! Request parameters for the start subcommand.

use crate::prelude::*;

/// Parsed arguments for the start subcommand.
#[derive(Args, Clone, Debug, Eq, PartialEq)]
pub struct StartRequest {
    /// Identifier of the resource to start.
    pub id: Id,
}

impl StartRequest {
    /// Create a new [`StartRequest`] from an [`Id`].
    #[cfg_attr(not(test), expect(dead_code, reason = "example"))]
    pub fn new(id: Id) -> Self {
        Self { id }
    }

    /// Create a mock [`StartRequest`] for testing.
    #[cfg(test)]
    pub fn mock() -> Self {
        Self::new(Id::mock())
    }
}
