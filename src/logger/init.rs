use std::str::FromStr;

use clap::ArgMatches;
use log::LevelFilter;
use log4rs::Handle;
use unwrap_or::{unwrap_ok_or, unwrap_some_or};

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
	fn init_logger(&self) -> Result<Handle>;

	/// # Init Logger Custom env
	/// Initialize the logger while reading from custom environment variable.
	///
	/// *Note: Requires non-default "custom_env" feature*
	#[cfg(feature = "custom_env")]
	fn init_logger_custom_env(&self, custom_env: &str) -> Result<Handle>;

	/// # Get loglevel
	/// TODO: Doc
	#[cfg(feature = "init_logger")]
	fn get_loglevel(&self) -> Result<LevelFilter>;

	/// TODO: Doc
	#[cfg(feature = "custom_env")]
	fn get_loglevel_custom_env(&self, custom_env: &str) -> Result<LevelFilter>;

	fn build(
		builder: fn(ClapLoggerBuilder) -> ClapLoggerBuilder,
		 default_loglevel: LevelFilter,
	) -> Result<Handle>;
}

impl ClapInitLogger for ArgMatches
where
	Self: Sized,
{
	#[cfg(feature = "init_logger")]
	fn init_logger(&self) -> Result<Handle> {
		let loglevel: LevelFilter = self.get_loglevel()?;

		Ok(ClapLoggerBuilder::new(loglevel)
			.add_console_logger()?
			.init()?)
	}

	#[cfg(feature = "custom_env")]
	fn init_logger_custom_env(&self, custom_env: impl Into<String>) -> Result<Handle> {
		let loglevel: LevelFilter = self.get_loglevel_custom_env(&custom_env.into())?;
		ClapLoggerBuilder::new(loglevel).add_console_logger().init()
	}

	fn get_loglevel(&self) -> Result<LevelFilter> {
		let loglevel: &str = unwrap_some_or!(
			self.value_of("loglevel"),
			return Err(Error::CouldntFindLoglevelArg)
		);

		let occurrences = |id: &str| -> Result<u64> {
			Ok(if self.value_of(id).is_none() {
				self.occurrences_of(id)
			} else {
				get_loglevel_index(
					LevelFilter::from_str(loglevel)
						.map_err(|e| Error::CouldntParseLoglevel { source: e })?,
				) as u64
			})
		};

		let verbose: u64 = occurrences("verbose")?;
		let quiet: u64 = occurrences("quiet")?;

		get_loglevel(loglevel, verbose, quiet, vec!["RUST_LOG"])
	}

	#[cfg(feature = "custom_env")]
	fn get_loglevel_custom_env(&self, custom_env: &str) -> Result<LevelFilter> {
		let loglevel: &str = unwrap_some_or!(
			self.value_of("loglevel"),
			return Err(Error::CouldntFindLoglevelArg)
		);

		let occurrences = |id: &str| -> u64 {
			if self.value_of("verbose").is_none() {
				self.occurrences_of("verbose")
			} else {
				get_loglevel_index(LevelFilter::from_str(loglevel).unwrap()) as u64
			}
		};

		let verbose: u64 = occurrences("verbose");
		let quiet: u64 = occurrences("quiet");

		Ok(get_loglevel_custom_env(
			loglevel, verbose, quiet, custom_env,
		)?)
	}

	fn build(
		builder: fn(ClapLoggerBuilder) -> ClapLoggerBuilder,
		default_loglevel: LevelFilter,
	) -> Result<Handle> {
		Ok(builder(ClapLoggerBuilder::new(default_loglevel)).init()?)
	}
}

/*#[doc(hidden)]
fn start_logger(loglevel: LevelFilter) -> Handle {
	let stdout: ConsoleAppender = ConsoleAppender::builder()
		.encoder(Box::new(PatternEncoder::new(
			"{l} – {d(%Y-%M-%d – %H-%m-%S-%2F)} – {m}\n",
		)))
		;
	let config: Config = Config::builder()
		.appender(Appender::builder().build("stdout", Box::new(stdout)))
		.build(Root::builder().appender("stdout").build(loglevel))
		.expect("Failed to build logger config");

	init_config(config).expect("failed to initialize logger")
}

#[cfg(feature = "logfile")]
enum LogFile {
	Rolling,
	Continuous,
}

#[cfg(feature = "logfile")]
enum LogFileBuilder {
	Rolling(RollingFileAppenderBuilder),
	Continuous(FileAppenderBuilder),
}

fn get_builders(
	loglevel: LevelFilter,
	logfile_path: Option<&str>,
	logfile_cfg: Option<LogFile>,
) -> (ConsoleAppenderBuilder, Option<LogFileBuilder>) {
	(
		ConsoleAppender::builder().encoder(Box::new(PatternEncoder::new(
			"{l} – {d(%Y-%M-%d – %H-%m-%S-%2F)} – {m}\n",
		))),
		match logfile {
			None => None,
			Some(LogFile::Continuous) => Some(LogFileBuilder::Continuous(
				FileAppender::builder()
					.encoder(Box::new(PatternEncoder::new(
						"{l} – {d(%Y-%M-%d – %H-%m-%S-%2F)} – {m}\n",
					)))
					.build(logfile_path),
			)),
			Some(LogFile::Rolling) => Some(RollingFileAppender),
		},
	)
}*/

#[doc(hidden)]
fn get_loglevel(
	default_loglevel: &str,
	verbose: u64,
	quiet: u64,
	env_vars: Vec<&str>,
) -> Result<LevelFilter> {
	let filter_index: usize = verbose.clamp(0, 5).saturating_sub(quiet) as usize;

	/*println!(
		"verbose: {}, quiet: {}, together: {}",
		verbose, quiet, filter_index
	);*/

	let loglevel: &str = LEVEL_FILTERS[filter_index];

	let mut loglevel_env: Option<String> = None;
	for var in env_vars {
		if loglevel_env == None {
			loglevel_env = match std::env::var(var) {
				Ok(r) => Some(r),
				Err(_) => None,
			}
		}
	}

	Ok(LevelFilter::from_str(loglevel).unwrap_or(unwrap_ok_or!(
		LevelFilter::from_str(default_loglevel),
		e,
		return Err(Error::CouldntParseLoglevel { source: e })
	)))
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