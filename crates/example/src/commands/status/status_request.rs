//! Request parameters for the status subcommand.

use crate::prelude::*;

/// Parsed arguments for the status subcommand.
#[derive(Args, Clone, Debug, Eq, PartialEq)]
pub struct StatusRequest {
    /// Identifier of the resource to query.
    pub id: Id,
}

impl StatusRequest {
    /// Create a [`StatusRequest`] fixture for testing.
    #[cfg(test)]
    pub fn fixture() -> Self {
        Self { id: Id::fixture() }
    }
}
