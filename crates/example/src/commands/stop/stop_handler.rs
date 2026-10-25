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

    /// [`StopHandler::execute`] with an active status.
    #[test]
    fn stop_handler_execute() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(StatusProviderStub {
                status: Ok(Status::Active),
            })
            .build();
        let handler = services.expect::<StopHandler>();
        let request = StopRequest::fixture();
        // Act
        let output = handler.execute(request);
        // Assert
        assert!(output.is_ok());
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
