//! Subcommand variants and dispatch handler.

use crate::prelude::*;
use clap::Subcommand as ClapSubcommand;

/// Available CLI subcommands.
#[derive(Clone, ClapSubcommand)]
pub enum Subcommand {
    /// Start the application.
    Start(StartRequest),
    /// Stop the application.
    Stop(StopRequest),
}

/// Dispatch the selected [`Subcommand`] to its handler.
#[derive(FromServices)]
pub struct SubCommandHandler {
    cli: Arc<CliArgs>,
    start: Arc<StartHandler>,
    stop: Arc<StopHandler>,
}

impl SubCommandHandler {
    /// Execute the selected subcommand.
    pub fn run(&self) -> Result<(), StructuredError> {
        let command = self.cli.command.clone();
        match command {
            Subcommand::Start(req) => Ok(self.start.execute(req)?),
            Subcommand::Stop(req) => Ok(self.stop.execute(req)?),
        }
    }
}
