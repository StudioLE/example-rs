//! Extension trait for configuring a [`ServiceBuilder`] with application services.

use crate::prelude::*;

/// Register all application services on a [`ServiceBuilder`].
pub trait ServiceBuilderExt {
    /// Register all application services.
    fn with_app_services(self) -> Self;

    /// Register all application services with stubs for external calls.
    #[cfg(test)]
    fn with_test_services(self) -> Self;
}

impl ServiceBuilderExt for ServiceBuilder {
    fn with_app_services(self) -> Self {
        self.with_logging(create_logger)
            .with_trait::<dyn WriteLine, ConsoleWriter>()
            .with_trait::<dyn StatusProvider, StatusFileAdapter>()
            .with_trait::<dyn StatusWriter, StatusFileAdapter>()
            .with_init::<StatusFileAdapter>()
            .with_type::<CliArgs>()
            .with_type::<Options>()
            .with_type::<StartHandler>()
            .with_type::<StatusHandler>()
            .with_type::<StopHandler>()
            .with_type::<SubcommandHandler>()
    }

    #[cfg(test)]
    fn with_test_services(self) -> Self {
        self.with_app_services()
            .with_trait::<dyn WriteLine, WriteLineSpy>()
            .with_trait::<dyn StatusProvider, StatusFake>()
            .with_trait::<dyn StatusWriter, StatusFake>()
            .with_instance(CliArgs::fixture())
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
#[expect(
    clippy::unnecessary_wraps,
    reason = "signature required by with_logging"
)]
fn create_logger(_services: &ServiceProvider) -> Result<Logger, Report<ResolveError>> {
    let logger = LoggerBuilder::new().with_level(LogLevel::Trace).build();
    Ok(logger)
}
