//! Application options resolved from CLI arguments and platform defaults.

use crate::prelude::*;
use etcetera::{AppStrategy, AppStrategyArgs, choose_app_strategy};

/// Application options resolved from [`CliArgs`] and platform defaults.
pub struct Options {
    /// Directory holding one status file per [`Id`].
    ///
    /// Default: the platform state dir, e.g. `~/.local/state/example`
    #[cfg_attr(not(test), expect(dead_code, reason = "example"))]
    pub state_dir: PathBuf,
}

impl FromServices for Options {
    type Error = OptionsError;

    fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        let args = services
            .get::<CliArgs>()
            .change_context(OptionsError::GetCliArgs)?;
        let state_dir = match &args.state_dir {
            Some(state_dir) => state_dir.clone(),
            None => get_default_state_dir()?,
        };
        Ok(Self { state_dir })
    }
}

/// Errors returned by [`Options::from_services`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum OptionsError {
    /// Unable to resolve [`CliArgs`].
    #[error("Unable to get CLI arguments")]
    GetCliArgs,
    /// Unable to locate the home dir to derive the default state dir.
    #[error("Unable to locate home directory")]
    HomeDir,
}

/// Get the platform state dir.
///
/// - Linux and macOS: `~/.local/state/<app_name>`
/// - Windows: `AppData\Roaming\<author>\<app_name>\data`
///
/// If `choose_native_strategy` were used:
/// - macOS: `~/Library/Application Support/<tld>.<author>.<app_name>`
fn get_default_state_dir() -> Result<PathBuf, Report<OptionsError>> {
    let strategy = choose_app_strategy(AppStrategyArgs {
        top_level_domain: String::from("uk"),
        author: String::from("StudioLE"),
        app_name: String::from(env!("CARGO_PKG_NAME")),
    })
    .change_context(OptionsError::HomeDir)?;
    Ok(strategy.state_dir().unwrap_or_else(|| strategy.data_dir()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [`Options::from_services`] with `--state-dir`.
    #[test]
    fn options_from_services() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_app_services()
            .with_instance(CliArgs {
                state_dir: Some(PathBuf::from("/var/tmp/example")),
                ..CliArgs::fixture()
            })
            .build();
        // Act
        let output = services.get::<Options>();
        // Assert
        let options = output.expect("should be able to resolve options");
        assert_eq!(options.state_dir, PathBuf::from("/var/tmp/example"));
    }
}
