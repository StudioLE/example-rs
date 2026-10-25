//! Handler for the start subcommand.

use crate::prelude::*;

/// Execute the start subcommand.
#[derive(FromServices)]
pub struct StartHandler {
    status: Arc<dyn StatusProvider>,
}

impl StartHandler {
    /// Start the application.
    pub fn execute(&self, request: StartRequest) -> Result<(), Report<StartError>> {
        trace!(?request, "Starting");
        let status = self
            .status
            .get(&request.id)
            .change_context(StartError::GetStatus)?;
        if status == Status::Active {
            return Err(Report::new(StartError::AlreadyActive).attach("id", request.id));
        }
        info!(id = %request.id, "Started");
        Ok(())
    }
}

/// Errors returned by [`StartHandler`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum StartError {
    /// Unable to get status.
    #[error("Unable to get status")]
    GetStatus,
    /// Already active.
    #[error("Already active")]
    AlreadyActive,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [`StartHandler::execute`] with an inactive status.
    #[test]
    fn start_handler_execute() {
        // Arrange
        let services = ServiceBuilder::new().with_test_services().build();
        let handler = services.expect::<StartHandler>();
        let request = StartRequest::fixture();
        // Act
        let output = handler.execute(request);
        // Assert
        assert!(output.is_ok());
    }
}
