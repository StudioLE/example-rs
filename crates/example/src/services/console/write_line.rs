//! Trait and spy for writing lines.

use crate::prelude::*;

/// Write lines of text.
pub trait WriteLine: Send + Sync {
    /// Write `line` followed by a newline.
    fn write_line(&self, line: &str) -> Result<(), Report<WriteLineError>>;
}

/// [`WriteLine`] that records each line and always succeeds.
#[cfg(test)]
#[derive(FromServices)]
pub struct WriteLineSpy {
    #[di(default)]
    lines: Mutex<Vec<String>>,
}

#[cfg(test)]
impl WriteLineSpy {
    /// Each line passed to [`WriteLine::write_line`], in call order.
    pub fn lines(&self) -> Vec<String> {
        self.lines
            .lock()
            .expect("should be able to lock lines")
            .clone()
    }
}

#[cfg(test)]
impl WriteLine for WriteLineSpy {
    fn write_line(&self, line: &str) -> Result<(), Report<WriteLineError>> {
        self.lines
            .lock()
            .expect("should be able to lock lines")
            .push(line.to_owned());
        Ok(())
    }
}

/// Errors returned by [`WriteLine`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum WriteLineError {
    /// Unable to write the line.
    #[error("Unable to write line")]
    Write,
}
