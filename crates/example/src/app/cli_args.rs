//! Parsed CLI arguments registered in the service container.
//!
//! - Declares each stdout format with `output(…)` so `--format` can select one
//! - Declares outputs on the root, as `select` must sit with its flag and every subcommand
//!   inherits them
//! - Carries `media_type` and `framing` into the usage spec for generated docs, SDKs, and
//!   `usage mcp`, with no effect at runtime
//! - Keeps these notes off [`CliArgs`], as usage renders its doc comment as long help

use crate::prelude::*;
use usage_rs::Cli as UsageCli;

/// Parsed CLI arguments.
#[derive(Debug, UsageCli)]
#[usage(
    bin = "example",
    about,
    version,
    unknown_flags = "error",
    output("human", default, help = "Human-readable text"),
    output("json", media_type = "application/json", framing = "json"),
    output("toml", media_type = "application/toml"),
    output("yaml", media_type = "application/yaml")
)]
pub struct CliArgs {
    /// Format to print output in.
    #[usage(long, global, env = "EXAMPLE_FORMAT", value_enum, select)]
    pub format: Option<OutputFormat>,
    /// Set the log level.
    #[usage(long, global, env = "EXAMPLE_LOG_LEVEL", value_enum)]
    pub log_level: Option<LogLevel>,
    /// Directory to store status files in.
    #[usage(long, global, env = "EXAMPLE_STATE_DIR", value_hint = usage_rs::ValueHint::DirPath)]
    pub state_dir: Option<PathBuf>,
    /// Subcommand to execute.
    #[usage(subcommand)]
    pub command: Subcommand,
}

impl CliArgs {
    /// Create a [`CliArgs`] fixture for testing.
    #[cfg(test)]
    pub fn fixture() -> Self {
        Self {
            format: None,
            log_level: None,
            state_dir: None,
            command: Subcommand::Start(StartRequest::fixture()),
        }
    }
}

impl FromServices for CliArgs {
    type Error = Infallible;

    fn from_services(_: &ServiceProvider) -> Result<Self, Report<Self::Error>>
    where
        Self: Sized,
    {
        Ok(CliArgs::parse())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use usage_rs::test::{Page, argv, help, parse};

    /// [`CliArgs::parse_from`] binds a log level and a subcommand.
    #[test]
    fn cli_args_parse_from() {
        // Arrange
        let input = argv(["--log-level", "debug", "start", "abc"]);
        let words = input.words();
        let spec = CliArgs::spec();
        // Act
        let cli = parse(spec, &words, CliArgs::parse_from).expect("the command line should parse");
        // Assert
        assert_eq!(cli.log_level, Some(LogLevel::Debug));
        assert_eq!(
            cli.command,
            Subcommand::Start(StartRequest::new(
                Id::from_str("abc").expect("should be able to parse id")
            ))
        );
    }

    /// [`CliArgs::parse_from`] binds the global `--state-dir` after a subcommand.
    #[test]
    fn cli_args_parse_from_state_dir() {
        // Arrange
        let input = argv(["status", "abc", "--state-dir", "/var/tmp/example"]);
        let words = input.words();
        let spec = CliArgs::spec();
        // Act
        let cli = parse(spec, &words, CliArgs::parse_from).expect("the command line should parse");
        // Assert
        assert_eq!(cli.state_dir, Some(PathBuf::from("/var/tmp/example")));
    }

    /// [`CliArgs::parse_from`] binds the global `--format` after a subcommand.
    #[test]
    fn cli_args_parse_from_format() {
        // Arrange
        let input = argv(["start", "abc", "--format", "json"]);
        let words = input.words();
        let spec = CliArgs::spec();
        // Act
        let cli = parse(spec, &words, CliArgs::parse_from).expect("the command line should parse");
        // Assert
        assert_eq!(cli.format, Some(OutputFormat::Json));
    }

    /// [`CliArgs::parse_from`] rejects an [`Id`] that isn't safe as a file name.
    #[test]
    fn cli_args_parse_from_invalid_id() {
        // Arrange
        let input = argv(["start", "../etc"]);
        let words = input.words();
        let spec = CliArgs::spec();
        // Act
        let message =
            parse(spec, &words, CliArgs::parse_from).expect_err("`../etc` is not a valid id");
        // Assert
        assert!(message.contains("Id must only contain"), "{message}");
    }

    /// [`CliArgs::parse_from`] rejects a log level outside the declared choices.
    #[test]
    fn cli_args_parse_from_invalid_log_level() {
        // Arrange
        let input = argv(["--log-level", "nonsense", "start", "abc"]);
        let words = input.words();
        let spec = CliArgs::spec();
        // Act
        let message =
            parse(spec, &words, CliArgs::parse_from).expect_err("`nonsense` is not a log level");
        // Assert
        assert!(message.contains("invalid value 'nonsense'"), "{message}");
    }

    /// [`CliArgs::parse_from`] rejects an unknown flag rather than binding it as an [`Id`].
    #[test]
    fn cli_args_parse_from_unknown_flag() {
        // Arrange
        let input = argv(["start", "--oops"]);
        let words = input.words();
        let spec = CliArgs::spec();
        // Act
        let message = parse(spec, &words, CliArgs::parse_from)
            .expect_err("`--oops` is not a flag of `start`");
        // Assert
        assert!(message.contains("--oops"), "{message}");
    }

    /// The root help page.
    ///
    /// - Replaces the version with a placeholder, as CI sets it at build time
    #[test]
    fn cli_args_help() {
        let spec = CliArgs::spec();
        let output = help(spec, &[], Page::Long).replace(env!("CARGO_PKG_VERSION"), "x.y.z");
        assert_snapshot!(output);
    }

    /// The help page for the start subcommand.
    #[test]
    fn cli_args_help_start() {
        let spec = CliArgs::spec();
        let output = help(spec, &["start"], Page::Long);
        assert_snapshot!(output);
    }

    /// The help page for the status subcommand.
    #[test]
    fn cli_args_help_status() {
        let spec = CliArgs::spec();
        let output = help(spec, &["status"], Page::Long);
        assert_snapshot!(output);
    }

    /// The help page for the stop subcommand.
    #[test]
    fn cli_args_help_stop() {
        let spec = CliArgs::spec();
        let output = help(spec, &["stop"], Page::Long);
        assert_snapshot!(output);
    }

    /// The usage spec printed by `example __usage_spec__`.
    ///
    /// - Replaces the version with a placeholder, as CI sets it at build time
    #[test]
    fn cli_args_to_kdl() {
        let output = CliArgs::to_kdl().replace(env!("CARGO_PKG_VERSION"), "x.y.z");
        assert_snapshot!(output);
    }
}
