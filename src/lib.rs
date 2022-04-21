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
//! * No Panics
//! * Argument can be modified
//! * Optional: Loglevel via Environment variables
//! * directly embedded in `clap::Command` and `clap::ArgMatches`
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
//!  	.add_loglevel_arg()
//! 	.get_matches();//!
//!
//! m.init_env_logger().expect("Failed to initialize logger");
//! ```
//! ## Status: Beta
//! ### Finished
//! * Feature complete (But Open for suggestions)
//! * no panics
//!
//! ### Backlog
//! * Write More tests
//! * Complete documentation
//! * Write more examples.
//! * Add `clap_derive` support.,

pub mod arg;
pub mod init_logger;
#[cfg(test)]
mod tests;

pub use crate::arg::{get_arg, ClapLoglevelArg};
pub use crate::init_logger::ClapInitLogger;

#[cfg(feature = "prelude")]
pub mod prelude {
	//! # Collection of imports for setting up the crate.
	//! Also re-exports clap and log commands needed for implementation to reduce imports and dependencies.
	//! [See start page for implementation details.][crate]
	//! Includes
	//! - basic clap modules (like [ArgMatches][clap::ArgMatches],[Command][clap::Command] and many more)
	//! - essential internal modules for setting up clap_logger
	//! - logging functions and LevelFilters
	pub use crate::arg::{get_arg, ClapLoglevelArg};
	pub use crate::init_logger::ClapInitLogger;
	pub use clap::{arg, command, Arg, ArgMatches, Command};
	pub use log::{debug, error, info, trace, warn, LevelFilter};
}

#[cfg(feature = "prelude")]
pub mod log {
	//! # Functions for logging.
	//! This provides a simple way to import logging functions without extra dependencies.
	pub use log::{debug, error, info, trace, warn};
}
