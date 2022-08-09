#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![doc = include_str!("lib.md")]

pub(crate) use crate::args::helper;
pub use crate::args::{ClapLogArgs, ClapLogArgsBuilder};
pub use crate::logger::{ClapInitLogger, ClapLoggerBuilder, FileLoggerType, PolicyBuilder};

pub mod args;
pub mod errors;
#[cfg(feature = "init_logger")]
pub mod logger;
#[cfg(feature = "prelude")]
pub mod prelude;
#[cfg(test)]
mod tests;

#[cfg(feature = "prelude")]
pub mod log {
	///  # Functions for logging.
	/// ```
	/// use clap_logger::log::*;
	/// ```
	/// This module provides the default logging functions without you needing to add extra dependencies.
	pub use log::{debug, error, info, trace, warn};
}
