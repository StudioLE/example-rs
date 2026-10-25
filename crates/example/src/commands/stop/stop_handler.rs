//! Handler for the stop subcommand.

use crate::prelude::*;

/// Execute the stop subcommand.
#[derive(FromServices)]
pub struct StopHandler {
    status_provider: Arc<dyn StatusProvider>,
    status_writer: Arc<dyn StatusWriter>,
}

impl StopHandler {
    /// Stop the application.
    pub fn execute(&self, request: StopRequest) -> Result<(), Report<StopError>> {
        trace!(?request, "Stopping");
        let status = self
            .status_provider
            .get(&request.id)
            .change_context(StopError::GetStatus)?;
        if status == Status::Inactive {
            return Err(Report::new(StopError::AlreadyInactive).attach("id", request.id));
        }
        self.status_writer
            .set(&request.id, Status::Inactive)
            .change_context(StopError::SetStatus)?;
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
    /// Unable to set status.
    #[error("Unable to set status")]
    SetStatus,
    /// Already inactive.
    #[error("Already inactive")]
    AlreadyInactive,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [`StopHandler::execute`] with an active status.
    #[test]
    fn stop_handler_execute() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(StatusFake::fixture_active())
            .build();
        let handler = services.expect::<StopHandler>();
        let request = StopRequest::fixture();
        // Act
        let output = handler.execute(request);
        // Assert
        assert!(output.is_ok());
        let fake = services.expect::<StatusFake>();
        assert_eq!(fake.get(&Id::fixture()).ok(), Some(Status::Inactive));
    }

    /// [`StopHandler::execute`] with an inactive status.
    #[test]
    fn stop_handler_execute_inactive() {
        // Arrange
        let services = ServiceBuilder::new().with_test_services().build();
        let handler = services.expect::<StopHandler>();
        let request = StopRequest::fixture();
        // Act
        let output = handler.execute(request);
        // Assert
        let report = output.expect_err("should not be able to stop when inactive");
        assert_eq!(report.current_context(), &StopError::AlreadyInactive);
    }
}
