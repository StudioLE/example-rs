//! Extension trait for configuring a [`ServiceBuilder`] with application services.

use crate::prelude::*;

/// Register all application services on a [`ServiceBuilder`].
pub trait ServiceBuilderExt {
    /// Register all application services.
    fn with_app_services(self) -> Self;

    /// Register a mock status provider and all application services.
    #[cfg(test)]
    fn with_mock_services(self, status_fn: fn(&Id) -> Result<Status, Report<StatusError>>) -> Self;
}

impl ServiceBuilderExt for ServiceBuilder {
    fn with_app_services(self) -> Self {
        self.with_logging(create_logger)
            .with_trait::<dyn StatusProvider, StatusProviderAdapter>()
            .with_type::<CliArgs>()
            .with_type::<StartHandler>()
            .with_type::<StopHandler>()
            .with_type::<SubCommandHandler>()
    }

    #[cfg(test)]
    fn with_mock_services(self, status_fn: fn(&Id) -> Result<Status, Report<StatusError>>) -> Self {
        let mut status = MockStatusProvider::new();
        status.expect_get().returning(status_fn);
        self.with_app_services()
            .with_instance::<Arc<dyn StatusProvider>>(Arc::new(status))
    }
}

#[cfg(not(test))]
fn create_logger(services: &ServiceProvider) -> Result<Logger, Report<ResolveError>> {
    let args = services.get::<CliArgs>()?;
    let logger = LoggerBuilder::new()
        .with_level(args.log_level.unwrap_or_default())
        .with_target("studiole_di::service_provider", LogLevel::Debug)
        .with_target(
            "studiole_di::traits::service_provider_get_trait",
            LogLevel::Debug,
        )
        .build();
    Ok(logger)
}

#[cfg(test)]
#[expect(clippy::unnecessary_wraps, reason = "signature required by with_logging")]
fn create_logger(_services: &ServiceProvider) -> Result<Logger, Report<ResolveError>> {
    let logger = LoggerBuilder::new().with_level(LogLevel::Trace).build();
    Ok(logger)
}
