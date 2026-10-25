//! Console writer for writing lines to stdout.

use crate::prelude::*;

/// [`WriteLine`] to stdout.
///
/// - Returns [`WriteLineError::Write`] on a broken pipe, where `println!` would panic
#[derive(Default, FromServices)]
pub struct ConsoleWriter;

impl WriteLine for ConsoleWriter {
    fn write_line(&self, line: &str) -> Result<(), Report<WriteLineError>> {
        let mut stdout = io::stdout().lock();
        writeln!(stdout, "{line}").change_context(WriteLineError::Write)
    }
}
