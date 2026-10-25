//! Output of the status subcommand.

use crate::prelude::*;

/// Current status of one [`Id`].
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StatusOutput {
    /// Identifier that was queried.
    pub id: Id,
    /// Current status.
    pub status: Status,
}

impl StatusOutput {
    /// Create a [`StatusOutput`] fixture for testing.
    #[cfg(test)]
    pub fn fixture() -> Self {
        Self {
            id: Id::fixture(),
            status: Status::fixture(),
        }
    }
}

impl Display for StatusOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.status)
    }
}
