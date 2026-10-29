//! Spy for writing lines.

use crate::prelude::*;

/// [`WriteLine`] that records each line and always succeeds.
#[derive(FromServices)]
pub struct WriteLineSpy {
    #[di(default)]
    lines: Mutex<Vec<String>>,
}

impl WriteLineSpy {
    /// Each line passed to [`WriteLine::write_line`], in call order.
    pub fn lines(&self) -> Vec<String> {
        self.lines
            .lock()
            .expect("should be able to lock lines")
            .clone()
    }
}

impl WriteLine for WriteLineSpy {
    fn write_line(&self, line: &str) -> Result<(), Report<WriteLineError>> {
        self.lines
            .lock()
            .expect("should be able to lock lines")
            .push(line.to_owned());
        Ok(())
    }
}
