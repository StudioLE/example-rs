//! Subcommand variants and dispatch handler.

use crate::prelude::*;
use usage_rs::Subcommands as UsageSubcommands;

/// Available CLI subcommands.
#[derive(Clone, Debug, Eq, PartialEq, UsageSubcommands)]
pub enum Subcommand {
    /// Start the application.
    Start(StartRequest),
    /// Stop the application.
    Stop(StopRequest),
}

/// Dispatch the selected [`Subcommand`] to its handler.
///
/// - Matches by hand because [`StartHandler::execute`] and [`StopHandler::execute`] return
///   different error types
/// - `usage` generates the match from `#[usage(run_with)]` when every command shares one `Output`
///   type: <https://usage.jdx.dev/rust/dispatch>
#[derive(FromServices)]
pub struct SubcommandHandler {
    cli: Arc<CliArgs>,
    start: Arc<StartHandler>,
    stop: Arc<StopHandler>,
}

impl SubcommandHandler {
    /// Execute the selected subcommand.
    pub fn run(&self) -> Result<(), StructuredError> {
        let command = self.cli.command.clone();
        match command {
            Subcommand::Start(req) => Ok(self.start.execute(req)?),
            Subcommand::Stop(req) => Ok(self.stop.execute(req)?),
        }
    }
}
