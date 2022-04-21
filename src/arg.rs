//! # Manage The loglevel
//! This modules provides the
//!
//! ## Adding the Argument
//! ### Base Implementation:
//! ```
//! use clap_logger::prelude::*;
//!
//! // Generate a clap command
//! let m: ArgMatches = Command::new("clap_command_test")//!
//!   // add loglevel argument
//!  	.add_loglevel_arg()
//! 	.get_matches();
//! ```
//!
//! ## loglevel Arg manipulation
//! You can also get the [Arg][clap::Arg] directly in order to modify it before adding:`
//! ```
//! use clap::{arg, Arg, Command};
//! use log::LevelFilter;
//! use clap_logger::prelude::*;
//!
//! // Generate a clap command
//! let m: clap::ArgMatches = Command::new("clap_command_test")
//!   // add the add loglevel argument
//!  	.arg(get_loglevel_arg(LevelFilter::Info)
//! 		// Adding a short version
//! 		.short('l')
//!     // changing the long version of the argument
//! 		.long("log")
//!     // make it required to annoy the user
//!     .required(true))
//! 	.get_matches();
//! ```
//! Warning: Do NOT touch [`.possible_values`][clap::], `.id` field of the argument or anything in that modifies the input.
//!
use log::LevelFilter;

pub mod get_arg {
	//! # Get an argument for logging
	//! get an Argument for logging.

	use clap::{Arg, PossibleValue};
	use log::LevelFilter;

	/// # Get LogLevel Arg
	/// Returns a [Arg][clap::Arg], which accepts the log level via CLI
	///
	/// ## Arguments
	/// default_loglevel: [LevelFilter][log::LevelFilter] which will become the loglevel when no one is defined by the user.
	///
	/// ## Priority
	///
	///

	fn loglevel_string(input: LevelFilter) -> String {
		input.to_string().to_lowercase()
	}

	pub fn loglevel<'a>(default_loglevel: LevelFilter) -> Arg<'a> {
		Arg::new("loglevel")
			.long("loglevel")
			.required(false)
			.default_value(
				// Cannot use &default_loglevel.to_String() because of borrow checker FIXME: find a way to use it.
				match default_loglevel {
					LevelFilter::Off => "off",
					LevelFilter::Error => "error",
					LevelFilter::Warn => "warn",
					LevelFilter::Info => "info",
					LevelFilter::Debug => "debug",
					LevelFilter::Trace => "trace",
				},
			)
			.help("Set the loglevel")
			.long_help("Set the loglevel. TRACE is the most verbose and OFF the least verbose")
			.possible_values([
				PossibleValue::new("off").help("Disable Logging completely"),
				PossibleValue::new("error").help("Only show Error messages"),
				PossibleValue::new("warn").help("Only show Warnings and Errors"),
				PossibleValue::new("info").help("Show Information, Warnings and Errors"),
				PossibleValue::new("debug").help("Show Debug information and upward messages"),
				PossibleValue::new("trace").help("Show all messages"),
			])
			.clone()
	}

	#[doc(hidden)]
	fn get_loglevel_difference(default_loglevel: LevelFilter) -> (usize, usize) {
		let level_filters: [&str; 6] = ["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"];
		let default_loglevel_pos: usize = level_filters
			.binary_search(&default_loglevel.as_str())
			.unwrap_or({
				println!("Verbose/Silent: Failed to get position of default loglevel, using Warn");
				2 as usize
			});

		(
			level_filters.len() - default_loglevel_pos,
			default_loglevel_pos - 1,
		)
	}

	/// # Verbose Arg
	/// [Arg][clap::Arg] to Increase loglevel
	pub fn verbose<'a>(default_loglevel: LevelFilter) -> Arg<'a> {
		let loglevel_difference: (usize, usize) = get_loglevel_difference(default_loglevel);
		let difference_to_max: usize = loglevel_difference.1;

		Arg::new("verbose")
			.short('v')
			.long("verbose")
			.help("Increase verbosity, It increases for each use")
			.multiple_occurrences(true)
			.max_occurrences(difference_to_max)
	}
}

/// # CLap Loglevel Argument
/// Trait which adds the loglevel argument.
///
/// Made for [`clap::Command`][clap::Command]
pub trait ClapLoglevelArg {
	fn add_loglevel_arg(self, default_loglevel: LevelFilter) -> Self;
}

impl ClapLoglevelArg for clap::Command<'_> {
	/// # Add Loglevel Argument
	/// Adds loglevel argument to the current [Command][clap::Command], which allows the user to easily change the loglevel.
	///
	/// ## Arguments
	/// default_loglevel: [LevelFilter][log::LevelFilter] which will become the loglevel when no one is defined by the user.
	fn add_loglevel_arg(self, default_loglevel: LevelFilter) -> Self {
		self.arg(get_arg::loglevel(default_loglevel))
	}
}
