//! Handler for the start subcommand.

use crate::prelude::*;

/// Execute the start subcommand.
#[derive(FromServices)]
pub struct StartHandler {
    status_provider: Arc<dyn StatusProvider>,
    status_writer: Arc<dyn StatusWriter>,
}

impl StartHandler {
    /// Start the application.
    pub fn execute(&self, request: StartRequest) -> Result<(), Report<StartError>> {
        trace!(?request, "Starting");
        let status = self
            .status_provider
            .get(&request.id)
            .change_context(StartError::GetStatus)?;
        if status == Status::Active {
            return Err(Report::new(StartError::AlreadyActive).attach("id", request.id));
        }
        self.status_writer
            .set(&request.id, Status::Active)
            .change_context(StartError::SetStatus)?;
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
    /// Unable to set status.
    #[error("Unable to set status")]
    SetStatus,
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
        let fake = services.expect::<StatusFake>();
        assert_eq!(fake.get(&Id::fixture()).ok(), Some(Status::Active));
    }

    /// [`StartHandler::execute`] with an active status.
    #[test]
    fn start_handler_execute_active() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(StatusFake::fixture_active())
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
            .with_trait::<dyn StatusProvider, StatusProviderStub>()
            .with_instance(StatusProviderStub {
                status: Err(StatusProviderError::Read),
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

    /// [`StartHandler::execute`] with a spy recording status writes.
    #[test]
    fn start_handler_execute_spy() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_trait::<dyn StatusWriter, StatusWriterSpy>()
            .build();
        let handler = services.expect::<StartHandler>();
        let request = StartRequest::fixture();
        // Act
        let output = handler.execute(request);
        // Assert
        output.expect("should be able to start");
        let spy = services.expect::<StatusWriterSpy>();
        let expected = vec![StatusWriterCall {
            id: Id::fixture(),
            status: Status::Active,
        }];
        assert_eq!(spy.calls(), expected);
    }
}
