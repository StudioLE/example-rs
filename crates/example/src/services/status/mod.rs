//! Application status types, traits, and adapters.

mod id;
mod status;
#[cfg(test)]
mod status_fake;
mod status_file_adapter;
mod status_output;
mod status_provider;
#[cfg(test)]
mod status_provider_stub;
mod status_writer;
#[cfg(test)]
mod status_writer_spy;

pub use id::*;
pub use status::*;
#[cfg(test)]
pub use status_fake::*;
pub use status_file_adapter::*;
pub use status_output::*;
pub use status_provider::*;
#[cfg(test)]
pub use status_provider_stub::*;
pub use status_writer::*;
#[cfg(test)]
pub use status_writer_spy::*;
