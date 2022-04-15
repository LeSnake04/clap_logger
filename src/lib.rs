//! # Clap Logger
//! simple [env_logger][env_logger] integration for [clap][clap].
//!
//! This create provides a simple way to allow the user to set the log level via a command line argument.
//! Its directly implemented in clap, so it feels very naturally.
//!
//! Please note this crate does not support `clap_derive` and won't support it in the near future or possibly never,
//! since Integrating with it is very hard to do.
//!
//! ## Features
//! * Command line argument to set loglevel
//! * Argument can be modified
//! * alternative
//!
//! ## Adding the Argument
//! ### Base Implementation:
//! ```
//! use clap::Command;
//! use log::LevelFilter;
//! use clap_logger::{ClapInitLogger, ClapLoglevelArg};
//!
//! // Generate a clap command
//! let m: clap::ArgMatches = Command::new("clap_command_test")//!
//!   // add loglevel argument
//!		.add_loglevel_arg()
//! 	.get_matches();
//! ```
//!
//! ## loglevel Arg manipulation
//! You can also get the [Arg][clap::Arg] individually in order to modify it before adding:`
//! ```
//! use clap::{arg, Arg, Command};
//! use log::LevelFilter;
//! use clap_logger::{ClapInitLogger, get_loglevel_arg};
//!
//! // Generate a clap command
//! let m: clap::ArgMatches = Command::new("clap_command_test")
//!   // add the add loglevel argument
//!  	.arg(get_loglevel_arg(LevelFilter::Info)
//! 		// Adding a short version
//! 		.short('l')
//!     // changing the long version of the argument just because I can
//! 		.long("custom-loglevel")
//!     // make it required to annoy the user
//!     .required(true))
//! 	.get_matches();
//! ```
//! Warning: Do NOT touch `.possible_values` or `.id` field of the argument  or enable multiple values and be careful not to modify the input in general.
//!
//! ## Initialising the logger
//!
//! ```
//! use clap::Command;
//! use log::LevelFilter;
//! use clap_logger::{ClapInitLogger, ClapLoglevelArg};
//!
//! let m: clap::ArgMatches = Command::new("clap_command_test")
//!   // add the loglevel argument
//!  	.add_loglevel_arg()
//! 	.get_matches();//!
//!
//! m.init_logger().expect("Failed to initialize logger");
//! ```
//! Note: if you didnt add

mod arg;
mod init_logger;
#[cfg(test)]
mod tests;

pub use log::{debug, error, info, trace, warn, LevelFilter};

pub use crate::arg::{get_loglevel_arg, ClapLoglevelArg};
pub use crate::init_logger::ClapInitLogger;
