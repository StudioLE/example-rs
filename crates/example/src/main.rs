//! Binary entrypoint for `example`.

use example::prelude::*;
use std::process::ExitCode;

fn main() -> ExitCode {
    Cli::new().run()
}
