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
//!  	.add_logging_args()
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
use clap::Command;
use log::LevelFilter;

pub mod get_logging_args {
	//! # Get an argument for logging
	//! get an Argument for logging.
	//! # How to use?
	//! Call `.add_logging_args()` on you clap command.
	//! ## or
	//! If you want to modify the args you have to add all these arguments of this modules:
	//! - `loglevel
	//! - `verbose,
	//! - `quiet
	//! Make sure you add all of them or clap initialization will fail.

	use clap::{Arg, PossibleValue};
	use log::LevelFilter;

	#[doc(hidden)]
	fn loglevel_string(input: LevelFilter) -> String {
		input.to_string().to_lowercase()
	}

	/// # Get LogLevel Arg
	/// Returns a [Arg][clap::Arg], which accepts the log level via CLI
	///
	/// ## Arguments
	/// default_loglevel: [LevelFilter][log::LevelFilter] which will become the loglevel when no one is defined by the user.
	///
	/// ## Priority
	///
	///
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
	pub(crate) static LEVEL_FILTERS: [&str; 6] = ["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

	#[doc(hidden)]
	pub(crate) fn get_loglevel_difference(default_loglevel: LevelFilter) -> (usize, usize) {
		let default_loglevel_pos: usize = LEVEL_FILTERS
			.binary_search(&default_loglevel.as_str())
			.unwrap_or({
				("clap_logger: Failed to get position of default loglevel, using Warn (This message will be hidden in release builds)");
				2 as usize
			})
			.clamp(0,5);

		(
			LEVEL_FILTERS
				.len()
				.clamp(0, 5)
				.saturating_sub(default_loglevel_pos),
			default_loglevel_pos.saturating_sub(1),
		)
	}

	#[doc(hidden)]
	pub(crate) fn get_loglevel_index(default_loglevel: LevelFilter) -> usize {
		LEVEL_FILTERS
			.binary_search(&default_loglevel.as_str())
			.unwrap_or({
				println!("Verbose/Silent: Failed to get position of default loglevel, using Warn");
				2 as usize
			})
			.clamp(0, 5)
	}

	/// # Verbose Arg
	/// [Arg][clap::Arg] to Increase loglevel
	pub fn verbose<'a>() -> Arg<'a> {
		Arg::new("verbose")
			.short('v')
			.long("verbose")
			.help("Increase verbosity, increases for each use")
			.multiple_occurrences(true)
	}

	/// TODO Doc
	pub fn quiet<'a>() -> Arg<'a> {
		Arg::new("quiet")
			.short('q')
			.long("quiet")
			.help("Decrease verbosity, decreases for each use")
			.multiple_occurrences(true)
	}
}

/// # CLap Loglevel Argument
/// Trait which adds the loglevel argument.
///
/// Made for [`clap::Command`][clap::Command]
pub trait ClapLogArgs {
	/// # Add Loglevel Argument
	/// Adds loglevel argument to the current [Command][clap::Command], which allows the user to easily change the loglevel.
	///
	/// ## Arguments
	/// default_loglevel: [LevelFilter][log::LevelFilter] which will become the loglevel when no one is defined by the user.
	///
	/// ## This
	/// TODO doc
	fn add_logging_args(self, default_loglevel: LevelFilter) -> Self;
	/// TODO Doc
	fn validate_logging(self) -> Self;
}

impl ClapLogArgs for Command<'_> {
	fn add_logging_args(self, default_loglevel: LevelFilter) -> Self {
		self.args(&[
			get_logging_args::loglevel(default_loglevel),
			get_logging_args::verbose(),
			get_logging_args::quiet(),
		])
	}

	/// TODO Doc
	fn validate_logging<'help>(self) -> Self {
		let log_args = ["loglevel", "verbose", "quiet"];
		let mut log_arg_bool: [bool; 3] = [false; 3];

		// Find out if
		for arg in self.get_arguments() {
			let i = log_args.binary_search(&arg.get_id());

			//
			if i.is_err() {
				break;
			}

			let i: usize = i.unwrap();
			if log_arg_bool[i] == false {
				log_arg_bool[i] = true
			} else {
				panic!(
					"[clap_logger] ERROR: Logging Argument '{}' found more than once.",
					arg.get_id()
				)
			}
		}

		self
	}
}
