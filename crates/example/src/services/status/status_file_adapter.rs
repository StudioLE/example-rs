//! File-backed adapter for reading and writing application status.

use crate::prelude::*;
use std::fs;

/// [`StatusProvider`] and [`StatusWriter`] that keep each [`Status`] in a file per [`Id`].
///
/// - Stores files in [`Options::state_dir`]
/// - Treats a missing file as [`Status::Inactive`]
#[derive(FromServices)]
pub struct StatusFileAdapter {
    options: Arc<Options>,
}

impl StatusFileAdapter {
    fn get_path(&self, id: &Id) -> PathBuf {
        self.options.state_dir.join(id.as_str())
    }
}

impl StatusProvider for StatusFileAdapter {
    fn get(&self, id: &Id) -> Result<Status, Report<StatusProviderError>> {
        let path = self.get_path(id);
        let contents = match fs::read_to_string(&path) {
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Status::Inactive),
            read => read
                .change_context(StatusProviderError::Read)
                .attach_path(&path)?,
        };
        Status::from_str(contents.trim())
            .change_context(StatusProviderError::Parse)
            .attach_path(&path)
    }
}

impl StatusWriter for StatusFileAdapter {
    fn set(&self, id: &Id, status: Status) -> Result<(), Report<StatusWriterError>> {
        let path = self.get_path(id);
        fs::write(&path, status.to_string())
            .change_context(StatusWriterError::Write)
            .attach_path(&path)
    }
}

impl Init for StatusFileAdapter {
    fn init(&self, _services: &ServiceProvider) -> Result<(), Report<InitError>> {
        fs::create_dir_all(&self.options.state_dir)
            .change_context(InitError::Init)
            .attach_path(&self.options.state_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// [`StatusFileAdapter::get`] for an [`Id`] that was never set.
    #[test]
    fn status_file_adapter_get() {
        // Arrange
        let temp = TempDir::new().expect("should be able to create temp dir");
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(Options {
                state_dir: temp.path().to_path_buf(),
            })
            .build();
        let adapter = services.expect::<StatusFileAdapter>();
        // Act
        let output = adapter.get(&Id::fixture());
        // Assert
        assert_eq!(output.ok(), Some(Status::Inactive));
    }

    /// [`StatusFileAdapter::get`] for a file holding an unknown value.
    #[test]
    fn status_file_adapter_get_unknown() {
        // Arrange
        let temp = TempDir::new().expect("should be able to create temp dir");
        fs::write(temp.path().join("fixture"), "Unknown").expect("should be able to write file");
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(Options {
                state_dir: temp.path().to_path_buf(),
            })
            .build();
        let adapter = services.expect::<StatusFileAdapter>();
        // Act
        let output = adapter.get(&Id::fixture());
        // Assert
        let report = output.expect_err("should not be able to parse status");
        assert_eq!(report.current_context(), &StatusProviderError::Parse);
    }

    /// [`StatusFileAdapter::set`] followed by [`StatusFileAdapter::get`].
    #[test]
    fn status_file_adapter_set() {
        // Arrange
        let temp = TempDir::new().expect("should be able to create temp dir");
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(Options {
                state_dir: temp.path().to_path_buf(),
            })
            .build();
        let adapter = services.expect::<StatusFileAdapter>();
        // Act
        let output = adapter.set(&Id::fixture(), Status::Active);
        // Assert
        assert!(output.is_ok());
        assert_eq!(adapter.get(&Id::fixture()).ok(), Some(Status::Active));
    }

    /// [`StatusFileAdapter::init`] when the state dir doesn't exist.
    #[test]
    fn status_file_adapter_init() {
        // Arrange
        let temp = TempDir::new().expect("should be able to create temp dir");
        let state_dir = temp.path().join("nested").join("state");
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(Options {
                state_dir: state_dir.clone(),
            })
            .build();
        let adapter = services.expect::<StatusFileAdapter>();
        // Act
        let output = adapter.init(&services);
        // Assert
        assert!(output.is_ok());
        assert!(state_dir.is_dir());
    }
}
