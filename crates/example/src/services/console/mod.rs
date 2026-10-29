//! Writing to the console.

mod console_writer;
mod write_line;
#[cfg(test)]
mod write_line_spy;

pub use console_writer::*;
pub use write_line::*;
#[cfg(test)]
pub use write_line_spy::*;
