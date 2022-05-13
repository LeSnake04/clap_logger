//! # Clap Logger
//! Simple [env_logger][env_logger] integration for [clap][clap].
//!
//! This create provides a simple way to allow the user to set the log level via a command line argument.
//! Its directly implemented in clap, so it feels very naturally.
//!
//! Please note this crate does not support `clap_derive` yet.
//!
//! ## Features
//! * Command line argument to set loglevel
//! * Argument can be modified
//! * Optional: Loglevel via Environment variables
//! * directly embedded in `clap::Command` and `clap::ArgMatches`
//! * no panics after successful initialization
//!
//! ## Initialising the logger
//! ### Base implementation:
//! ```
//! use clap::Command;
//! use log::LevelFilter;
//! use clap_logger::prelude::*;
//!
//! let m: clap::ArgMatches = Command::new("clap_command_test")
//!   // add the loglevel argument
//!  	.add_logging_args()
//! 	.get_matches();//!
//!
//! m.init_logger().expect("Failed to initialize logger");
//! ```
//! ## Status: Beta
//!
//! ### Roadmap: 0.5
//! * More tests
//! * Complete documentation
//! * More examples.
//! * `clap_derive` support
//!
//! ## Note:
//! 1. If you get a panic or error ending with `"Please report!"`, this is very likely because of a bug in the library.
//! Please report the panic message on GitHub https://github.com/LeSnake04/clap_logger/issues
//!
//! 2. If you get a message ending with `'*'`, the message will be hidden in release builds.

pub use crate::arg::{get_logging_args, ClapLogArgs};
pub use crate::logger::init::ClapInitLogger;
use std::fmt::Display;

pub mod arg;
#[cfg(feature = "init_logger")]
pub mod logger;
#[cfg(test)]
mod tests;

#[doc(hidden)]
pub(crate) fn print_dbg(text: impl Display) {
	#[cfg(debug_assertions)]
	println!("{}", text);
}

#[cfg(feature = "prelude")]
pub mod prelude {
	//! ```
	//! use clap_logger::prelude::*;
	//! ```
	//! # Collection of imports for setting up the crate.
	//! Also re-exports clap and log commands needed for implementation to reduce imports and dependencies.
	//! [See start page for implementation details.][crate]
	//! Includes
	//! - essential modules for setting up clap_logger
	//! - basic clap modules (like [ArgMatches][clap::ArgMatches],[Command][clap::Command] and many more)
	//! - logging functions and LevelFilters

	pub use clap::{arg, command, Arg, ArgMatches, Command};
	pub use log::LevelFilter;
	#[cfg(feature = "init_logger")]
	pub use log4rs::Handle;

	pub use crate::arg::{get_logging_args, ClapLogArgs};
	pub use crate::log::*;
	pub use crate::ClapInitLogger;
}

#[cfg(feature = "prelude")]
pub mod log {
	//! ```
	//! use clap_logger::log::*;
	//! ```
	//! # Functions for logging.
	//! This provides a simple way to import logging functions without extra dependencies.
	pub use log::{debug, error, info, trace, warn};
}

/// Internal use
pub(crate) fn dbgm(msg: impl Display) {
	#[cfg(debug_assertions)]
	print!("{}", msg);
	println!("*")
}
