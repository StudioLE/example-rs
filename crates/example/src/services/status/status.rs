//! Application status enum.

use crate::prelude::*;

/// Current application status.
#[derive(Clone, Copy, Debug, Deserialize, Display, EnumString, Eq, PartialEq, Serialize)]
pub enum Status {
    Active,
    Inactive,
}

impl Status {
    /// Create a [`Status`] fixture for testing.
    #[cfg(test)]
    pub fn fixture() -> Self {
        Self::Inactive
    }
}
