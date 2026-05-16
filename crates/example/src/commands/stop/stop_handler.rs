//! Handler for the stop subcommand.

use crate::prelude::*;

/// Execute the stop subcommand.
#[derive(FromServices)]
pub struct StopHandler {
    status: Arc<dyn StatusProvider>,
}

impl StopHandler {
    /// Stop the application.
    pub fn execute(&self, request: StopRequest) -> Result<(), Report<StopError>> {
        trace!(?request, "Stopping");
        let status = self
            .status
            .get(&request.id)
            .change_context(StopError::GetStatus)?;
        if status == Status::Inactive {
            return Err(Report::new(StopError::AlreadyInactive).attach("id", request.id));
        }
        info!(id = %request.id, "Stopped");
        Ok(())
    }
}

/// Errors returned by [`StopHandler`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum StopError {
    /// Unable to get status.
    #[error("Unable to get status")]
    GetStatus,
    /// Already inactive.
    #[error("Already inactive")]
    AlreadyInactive,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [`StopHandler::execute`] succeeds with mock status.
    #[test]
    fn stop_handler_execute() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_mock_services(|_| Ok(Status::Active))
            .build();
        let handler = services.expect::<StopHandler>();
        let request = StopRequest::mock();
        // Act
        let output = handler.execute(request);
        // Assert
        assert!(output.is_ok());
    }
}
