//! Common imports used throughout the crate.
#![allow(unused_imports, reason = "prelude re-exports for convenience")]

pub use crate::app::Cli;
pub(crate) use crate::app::*;
pub(crate) use crate::commands::*;
pub(crate) use crate::services::*;

pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
pub(crate) use std::convert::Infallible;
pub(crate) use std::error::Error;
pub(crate) use std::fmt::{self, Debug, Display, Formatter};
pub(crate) use std::io::{self, Read, Write};
pub(crate) use std::path::{Path, PathBuf};
pub(crate) use std::str::FromStr;
pub(crate) use std::sync::{Arc, Mutex};
pub(crate) use strum::{Display, EnumString};
pub(crate) use studiole_di::prelude::*;
pub(crate) use studiole_logging::prelude::*;
pub(crate) use studiole_report::prelude::*;
pub(crate) use thiserror::Error;
pub(crate) use tracing::{debug, error, info, trace, warn};
pub(crate) use usage_rs::{Args, ValueEnum};

#[cfg(test)]
pub(crate) use insta::{assert_snapshot, assert_yaml_snapshot};
