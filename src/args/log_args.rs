#![doc = include_str!("log_args.md")]

use clap::Command;
use log::LevelFilter;

// use crate::errors::{ClapLogArgsError as Error, ClapLogArgsResult as Result};
use crate::ClapLogArgsBuilder;

/// # CLap Loglevel Argument
/// Trait which adds the loglevel argument.
///
/// Made for [`clap::Command`][clap::Command]
///
/// # How to add logging Arguments
/// Call `.add_logging_args(...)` on you clap command or
/// if you want to modify the args, use `.add_modified_logging_args(...)`
pub trait ClapLogArgs {
	/// # Add Loglevel Argument
	/// Adds loglevel argument to the current [Command][clap::Command], which allows the user to easily change the loglevel.
	///
	/// ## Arguments
	/// default_loglevel: [LevelFilter][log::LevelFilter] which will become the loglevel when no one is defined by the user.
	///
	/// ## This Trait requires [ClapInitLogger][crate::ClapInitLogger] to be functional
	fn add_logging_args(self, default_loglevel: LevelFilter) -> Self;
	/// # Build Logging Args
	/// TODO: Doc
	/// TODO: Examples
	fn build_logging_args(
		self,
		default_loglevel: LevelFilter,
		args: fn(ClapLogArgsBuilder) -> ClapLogArgsBuilder,
	) -> Self;
}

impl ClapLogArgs for Command<'_> {
	fn add_logging_args(self, default_loglevel: LevelFilter) -> Self {
		self.args(ClapLogArgsBuilder::new(default_loglevel).export())
	}
	fn build_logging_args(
		self,
		default_loglevel: LevelFilter,
		args: fn(ClapLogArgsBuilder) -> ClapLogArgsBuilder,
	) -> Self {
		self.args(args(ClapLogArgsBuilder::new(default_loglevel)).export())
	}
}
