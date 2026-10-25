//! Subcommand variants and dispatch handler.

use crate::prelude::*;
use usage_rs::Subcommands as UsageSubcommands;

/// Available CLI subcommands.
#[derive(Clone, Debug, Eq, PartialEq, UsageSubcommands)]
pub enum Subcommand {
    /// Start the application.
    Start(StartRequest),
    /// Print the application status.
    Status(StatusRequest),
    /// Stop the application.
    Stop(StopRequest),
}

/// Dispatch the selected [`Subcommand`] to its handler.
///
/// - Matches by hand because each handler returns a different error type
/// - `usage` generates the match from `#[usage(run_with)]` when every command shares one `Output`
///   type: <https://usage.jdx.dev/rust/dispatch>
#[derive(FromServices)]
pub struct SubcommandHandler {
    cli: Arc<CliArgs>,
    start: Arc<StartHandler>,
    status: Arc<StatusHandler>,
    stop: Arc<StopHandler>,
}

impl SubcommandHandler {
    /// Execute the selected subcommand.
    pub fn run(&self) -> Result<(), StructuredError> {
        let command = self.cli.command.clone();
        match command {
            Subcommand::Start(req) => Ok(self.start.execute(req)?),
            Subcommand::Status(req) => {
                self.status.execute(req)?;
                Ok(())
            }
            Subcommand::Stop(req) => Ok(self.stop.execute(req)?),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [`SubcommandHandler::run`] with the status subcommand.
    #[test]
    fn subcommand_handler_run() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(CliArgs {
                command: Subcommand::Status(StatusRequest::fixture()),
                ..CliArgs::fixture()
            })
            .build();
        let handler = services.expect::<SubcommandHandler>();
        // Act
        let output = handler.run();
        // Assert
        assert!(output.is_ok());
    }
}
