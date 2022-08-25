use std::str::FromStr;

use clap::ArgMatches;
use log::LevelFilter;

use crate::args::helper::{get_loglevel_index, LEVEL_FILTERS};
use crate::errors::{ClapInitLoggerError as Error, ClapInitLoggerResult as Result};
use crate::logger::builder::ClapLoggerBuilder;

/// # Clap Init Logger
/// Trait which defines the functions to initializes the logger or get the loglevel
///
/// Made for [ArgMatches][clap::ArgMatches]
///
/// You need to add [ClapLogArgs][crate::ClapLogArgs] to [Command][clap::Command] use this
pub trait ClapInitLogger
where
	Self: Sized,
{
	/// TODO: Doc
	#[cfg(feature = "init_logger")]
	fn init_logger(&self) -> Result<()>;

	/// # Init Logger Custom env
	/// Initialize the logger while reading from custom environment variable.
	///
	/// *Note: Requires non-default "custom_env" feature*
	#[cfg(feature = "custom_env")]
	fn init_logger_custom_env(&self, custom_env: impl Into<String>) -> Result<()>;

	#[cfg(feature = "init_logger")]
	#[doc = "# Get loglevel\n TODO: Doc"]
	#[doc = include_str!("doc/loglevel_order.md")]
	fn get_loglevel(&self) -> Result<LevelFilter>;

	/// TODO: Doc
	#[cfg(feature = "custom_env")]
	fn get_loglevel_custom_env(&self, custom_env: &str) -> Result<LevelFilter>;

	#[cfg(feature = "init_logger")]
	/// TODO: Doc
	fn build_logger(
		&self,
		default_loglevel: LevelFilter,
		builder: impl FnOnce(ClapLoggerBuilder) -> ClapLoggerBuilder,
	) -> Result<()>;
}

impl ClapInitLogger for ArgMatches
where
	Self: Sized,
{
	#[cfg(feature = "init_logger")]
	fn init_logger(&self) -> Result<()> {
		let loglevel: LevelFilter = self.get_loglevel()?;

		self.build_logger(loglevel, |builder| builder)
	}

	#[cfg(feature = "custom_env")]
	fn init_logger_custom_env(&self, custom_env: impl Into<String>) -> Result<()> {
		let loglevel: LevelFilter = self.get_loglevel_custom_env(&custom_env.into())?;
		self.build_logger(loglevel, |builder| builder)
	}

	fn get_loglevel(&self) -> Result<LevelFilter> {
		let loglevel: String = self
			.value_of("loglevel")
			.map(|l| l.to_uppercase())
			.ok_or(Error::CouldntFindLoglevelArg)?;

		let occurrences = |id: &str| -> Result<u64> {
			Ok(if self.value_of(id).is_none() {
				self.occurrences_of(id)
			} else {
				get_loglevel_index(
					LevelFilter::from_str(&loglevel)
						.map_err(|e| Error::CouldntParseLoglevel { source: e })?,
				)
				.map(|i| i as u64)?
			})
		};

		let verbose: u64 = occurrences("verbose")?;
		let quiet: u64 = occurrences("quiet")?;

		get_loglevel(&loglevel, verbose, quiet, vec!["RUST_LOG"])
	}

	#[cfg(feature = "custom_env")]
	fn get_loglevel_custom_env(&self, custom_env: &str) -> Result<LevelFilter> {
		let loglevel: String = self
			.value_of("loglevel")
			.map(|l| l.to_uppercase())
			.ok_or(Error::CouldntFindLoglevelArg)?;

		let occur = |id: &str| -> u64 { self.occurrences_of(id) };

		let verbose: u64 = occur("verbose");
		let quiet: u64 = occur("quiet");

		get_loglevel_custom_env(&loglevel, verbose, quiet, custom_env)
	}

	#[cfg(feature = "init_logger")]
	fn build_logger(
		&self,
		default_loglevel: LevelFilter,
		builder: impl FnOnce(ClapLoggerBuilder) -> ClapLoggerBuilder,
	) -> Result<()> {
		Ok(builder(ClapLoggerBuilder::new(default_loglevel)).init()?)
	}
}

#[doc(hidden)]
fn get_loglevel(
	default_loglevel: &str,
	verbose: u64,
	quiet: u64,
	env_vars: Vec<&str>,
) -> Result<LevelFilter> {
	let filter_index: usize = verbose.clamp(0, 5).saturating_sub(quiet) as usize;

	let loglevel: &str = LEVEL_FILTERS
		.get(filter_index)
		.ok_or(Error::InvalidLoglevelIndex {
			index: filter_index,
		})?;

	let mut loglevel_env: Option<String> = None;
	for var in env_vars {
		if loglevel_env.is_none() {
			loglevel_env = match std::env::var(var) {
				Ok(r) => Some(r),
				Err(_) => None,
			}
		}
	}

	Ok(LevelFilter::from_str(loglevel).unwrap_or(
		LevelFilter::from_str(default_loglevel)
			.map_err(|e| Error::CouldntParseLoglevel { source: e })?,
	))
}

#[cfg(feature = "custom_env")]
#[doc(hidden)]
fn get_loglevel_custom_env(
	loglevel: &str,
	verbose: u64,
	quiet: u64,
	custom_env: &str,
) -> Result<LevelFilter> {
	get_loglevel(loglevel, verbose, quiet, vec![custom_env, "RUST_LOG"])
}