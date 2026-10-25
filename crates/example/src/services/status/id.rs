//! Unique identifier for a status query.

use crate::prelude::*;

/// Identifier safe to use as a file name.
///
/// - Contains only ASCII letters, digits, `-`, and `_`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Id(String);

impl Id {
    /// The inner string slice.
    #[cfg_attr(not(test), expect(dead_code, reason = "example"))]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the [`Id`] and return the inner string.
    #[expect(dead_code, reason = "example")]
    pub fn into_string(self) -> String {
        self.0
    }

    /// Create an [`Id`] fixture for testing.
    #[cfg(test)]
    pub fn fixture() -> Self {
        Self(String::from("fixture"))
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for Id {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(IdError::Empty);
        }
        let is_valid = s
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
        if !is_valid {
            return Err(IdError::InvalidCharacter);
        }
        Ok(Self(s.to_owned()))
    }
}

/// Errors returned by [`Id::from_str`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum IdError {
    /// The value is empty.
    #[error("Id must not be empty")]
    Empty,
    /// The value contains a character other than ASCII letters, digits, `-`, or `_`.
    #[error("Id must only contain ASCII letters, digits, `-`, or `_`")]
    InvalidCharacter,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [`Id::from_str`] with letters, digits, `-`, and `_`.
    #[test]
    fn id_from_str() {
        let output = Id::from_str("abc-123_x");
        let id = output.expect("should be able to parse id");
        assert_eq!(id.as_str(), "abc-123_x");
    }

    /// [`Id::from_str`] with an empty value.
    #[test]
    fn id_from_str_empty() {
        let output = Id::from_str("");
        assert_eq!(output, Err(IdError::Empty));
    }

    /// [`Id::from_str`] with a path traversal.
    #[test]
    fn id_from_str_path() {
        let output = Id::from_str("../etc");
        assert_eq!(output, Err(IdError::InvalidCharacter));
    }
}
