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

    /// [`StartHandler::execute`] with an active status.
    #[test]
    fn start_handler_execute_active() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(StatusProviderStub {
                status: Ok(Status::Active),
            })
            .build();
        let handler = services.expect::<StartHandler>();
        let request = StartRequest::fixture();
        // Act
        let output = handler.execute(request);
        // Assert
        let report = output.expect_err("should not be able to start when active");
        assert_eq!(report.current_context(), &StartError::AlreadyActive);
    }

    /// [`StartHandler::execute`] when the status can't be read.
    #[test]
    fn start_handler_execute_status_error() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(StatusProviderStub {
                status: Err(StatusError::Read),
            })
            .build();
        let handler = services.expect::<StartHandler>();
        let request = StartRequest::fixture();
        // Act
        let output = handler.execute(request);
        // Assert
        let report = output.expect_err("should not be able to start without status");
        assert_eq!(report.current_context(), &StartError::GetStatus);
    }

    /// [`StartHandler::execute`] with a spy recording status queries.
    #[test]
    fn start_handler_execute_spy() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_trait::<dyn StatusProvider, StatusProviderSpy>()
            .build();
        let handler = services.expect::<StartHandler>();
        let request = StartRequest::fixture();
        // Act
        let output = handler.execute(request.clone());
        // Assert
        output.expect("should be able to start");
        let spy = services.expect::<StatusProviderSpy>();
        assert_eq!(spy.calls(), vec![request.id]);
    }
}
