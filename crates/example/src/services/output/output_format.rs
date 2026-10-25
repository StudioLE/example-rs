//! Format for rendering command output.

use crate::prelude::*;

/// Format for rendering command output.
#[derive(Clone, Copy, Debug, Default, Display, Eq, PartialEq, ValueEnum)]
#[strum(serialize_all = "lowercase")]
pub enum OutputFormat {
    /// Human-readable text.
    #[default]
    Human,
    /// JSON.
    Json,
    /// TOML.
    Toml,
    /// YAML.
    Yaml,
}
