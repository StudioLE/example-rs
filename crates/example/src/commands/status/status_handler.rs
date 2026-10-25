//! Handler for the status subcommand.

use crate::prelude::*;

/// Execute the status subcommand.
#[derive(FromServices)]
pub struct StatusHandler {
    console: Arc<dyn WriteLine>,
    renderer: Arc<OutputRenderer>,
    status: Arc<dyn StatusProvider>,
}

impl StatusHandler {
    /// Print the current status.
    pub fn execute(
        &self,
        request: StatusRequest,
    ) -> Result<StatusOutput, Report<StatusHandlerError>> {
        trace!(?request, "Getting status");
        let status = self
            .status
            .get(&request.id)
            .change_context(StatusHandlerError::GetStatus)?;
        let output = StatusOutput {
            id: request.id,
            status,
        };
        let rendered = self
            .renderer
            .render(&output)
            .change_context(StatusHandlerError::Render)?;
        self.console
            .write_line(&rendered)
            .change_context(StatusHandlerError::Write)?;
        Ok(output)
    }
}

/// Errors returned by [`StatusHandler`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum StatusHandlerError {
    /// Unable to get status.
    #[error("Unable to get status")]
    GetStatus,
    /// Unable to render the output.
    #[error("Unable to render output")]
    Render,
    /// Unable to write the output.
    #[error("Unable to write output")]
    Write,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [`StatusHandler::execute`] with an active status.
    #[test]
    fn status_handler_execute() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(StatusFake::fixture_active())
            .build();
        let handler = services.expect::<StatusHandler>();
        let spy = services.expect::<WriteLineSpy>();
        let request = StatusRequest::fixture();
        let expected = StatusOutput {
            status: Status::Active,
            ..StatusOutput::fixture()
        };
        // Act
        let output = handler.execute(request);
        // Assert
        assert_eq!(output.ok(), Some(expected));
        assert_eq!(spy.lines(), vec![String::from("fixture: Active")]);
    }
}
