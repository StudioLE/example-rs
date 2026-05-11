//! Unique identifier for a status query.

use crate::prelude::*;

/// Opaque identifier wrapping a string value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Id(String);

impl Id {
    /// Create a new [`Id`] from a string value.
    #[cfg_attr(not(test), expect(dead_code, reason = "example"))]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// The inner string slice.
    #[expect(dead_code, reason = "example")]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the [`Id`] and return the inner string.
    #[expect(dead_code, reason = "example")]
    pub fn into_string(self) -> String {
        self.0
    }

    /// Create a mock [`Id`] for testing.
    #[cfg(test)]
    pub fn mock() -> Self {
        Self::new("mock")
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.0)
    }
}

impl FromStr for Id {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}
