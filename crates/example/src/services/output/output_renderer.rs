//! Render command output in the format selected by `--format`.

use crate::prelude::*;

/// Render command output in the [`OutputFormat`] selected by [`CliArgs::format`].
#[derive(FromServices)]
pub struct OutputRenderer {
    cli: Arc<CliArgs>,
}

impl OutputRenderer {
    /// Render `value` without trailing whitespace.
    pub fn render<T>(&self, value: &T) -> Result<String, Report<OutputRendererError>>
    where
        T: Display + Serialize,
    {
        let format = self.cli.format.unwrap_or_default();
        let output = match format {
            OutputFormat::Human => Ok(value.to_string()),
            OutputFormat::Json => serde_json::to_string_pretty(value)
                .change_context(OutputRendererError::Serialize)
                .attach("format", format),
            OutputFormat::Toml => toml::to_string(value)
                .change_context(OutputRendererError::Serialize)
                .attach("format", format),
            OutputFormat::Yaml => serde_yaml::to_string(value)
                .change_context(OutputRendererError::Serialize)
                .attach("format", format),
        }?;
        Ok(output.trim_end().to_owned())
    }
}

/// Errors returned by [`OutputRenderer`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum OutputRendererError {
    /// Unable to serialize the value.
    #[error("Unable to serialize output")]
    Serialize,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [`OutputRenderer::render`] as human-readable text.
    #[test]
    fn output_renderer_render_human() {
        // Arrange
        let services = ServiceBuilder::new().with_test_services().build();
        let renderer = services.expect::<OutputRenderer>();
        // Act
        let output = renderer.render(&StatusOutput::fixture());
        // Assert
        assert_snapshot!(output.expect("should be able to render"));
    }

    /// [`OutputRenderer::render`] as JSON.
    #[test]
    fn output_renderer_render_json() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(CliArgs {
                format: Some(OutputFormat::Json),
                ..CliArgs::fixture()
            })
            .build();
        let renderer = services.expect::<OutputRenderer>();
        // Act
        let output = renderer.render(&StatusOutput::fixture());
        // Assert
        assert_snapshot!(output.expect("should be able to render"));
    }

    /// [`OutputRenderer::render`] as TOML.
    #[test]
    fn output_renderer_render_toml() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(CliArgs {
                format: Some(OutputFormat::Toml),
                ..CliArgs::fixture()
            })
            .build();
        let renderer = services.expect::<OutputRenderer>();
        // Act
        let output = renderer.render(&StatusOutput::fixture());
        // Assert
        assert_snapshot!(output.expect("should be able to render"));
    }

    /// [`OutputRenderer::render`] as YAML.
    #[test]
    fn output_renderer_render_yaml() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(CliArgs {
                format: Some(OutputFormat::Yaml),
                ..CliArgs::fixture()
            })
            .build();
        let renderer = services.expect::<OutputRenderer>();
        // Act
        let output = renderer.render(&StatusOutput::fixture());
        // Assert
        assert_snapshot!(output.expect("should be able to render"));
    }
}
