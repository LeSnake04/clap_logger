use clap::ArgMatches;
use log4rs::append::console::{ConsoleAppender, ConsoleAppenderBuilder};
use log4rs::append::file::{FileAppender, FileAppenderBuilder};
use log4rs::append::rolling_file::{RollingFileAppender, RollingFileAppenderBuilder};
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::{init_config, Config, Handle};
use std::str::FromStr;

use log::LevelFilter;

use crate::get_logging_args::{get_loglevel_index, LEVEL_FILTERS};
use crate::logger::builder::ClapLoggerBuilder;

/// # Clap Init Logger
/// Trait which defines the functions to [initializes the logger][crate::init_logger] or get the loglevel
pub trait ClapInitLogger {
	///  TODO Doc
	#[cfg(feature = "init_logger")]
	fn init_logger(self) -> Handle
	where
		Self: Sized;

	/*#[cfg(feature = "init_logger")]
	fn init_logger_crate_only(self) -> Self
	where
		Self: Sized;*/

	/// TODO Doc
	#[cfg(feature = "from_custom_env")]
	fn init_logger_custom_env(self, custom_env: &str) -> Handle
	where
		Self: Sized;
}

impl ClapInitLogger for ArgMatches {
	#[cfg(feature = "init_logger")]

	fn init_logger(self) -> Handle {
		let arg_loglevel: &str = self
			.value_of("loglevel")
			.expect("Could not find loglevel argument. Please make sure you added the ar");

		let occurrences = |id: &str| -> u64 {
			if self.value_of(id).is_none() {
				self.occurrences_of(id)
			} else {
				get_loglevel_index(LevelFilter::from_str(arg_loglevel).unwrap()) as u64
			}
		};

		let verbose: u64 = occurrences("verbose");
		let quiet: u64 = occurrences("quiet");

		let loglevel: LevelFilter = get_loglevel(arg_loglevel, verbose, quiet, vec!["RUST_LOG"]);

		/*env_logger::builder()
		.filter_level(get_loglevel(loglevel, verbose, quiet, vec!["RUST_LOG"]))
		.init();*/
		ClapLoggerBuilder::new(loglevel).console_logger().init()
	}

	#[cfg(feature = "from_custom_env")]
	fn init_logger_custom_env(self, custom_env: &str) -> Handle {
		let arg_loglevel: &str = self
			.value_of("loglevel")
			.expect("Could not find loglevel argument. Please make sure you added the Command");

		let occurrences = |id: &str| -> u64 {
			if self.value_of("verbose").is_none() {
				self.occurrences_of("verbose")
			} else {
				get_loglevel_index(LevelFilter::from_str(arg_loglevel).unwrap()) as u64
			}
		};

		let verbose: u64 = occurrences("verbose");
		let quiet: u64 = occurrences("quiet");

		let loglevel: LevelFilter =
			get_loglevel_custom_env(env_loglevel_handling, verbose, quiet, custom_env);

		ClapLoggerBuilder::new(loglevel).console_logger().init()
	}
}

/*#[doc(hidden)]
fn start_logger(loglevel: LevelFilter) -> Handle {
	let stdout: ConsoleAppender = ConsoleAppender::builder()
		.encoder(Box::new(PatternEncoder::new(
			"{l} – {d(%Y-%M-%d – %H-%m-%S-%2F)} – {m}\n",
		)))
		.build();
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
) -> LevelFilter {
	let filter_index: usize = verbose.clamp(0, 5).saturating_sub(quiet) as usize;

	/*println!(
		"verbose: {}, quiet: {}, together: {}",
		verbose, quiet, filter_index
	);*/

	let loglevel: &str = LEVEL_FILTERS[filter_index];

	let mut loglevel_env: Option<String> = None;
	for i in 0..env_vars.len() {
		let var: &str = env_vars[i];
		if loglevel_env == None {
			loglevel_env = match std::env::var(var) {
				Ok(r) => Some(r),
				Err(_) => None,
			}
		}
	}

	LevelFilter::from_str(loglevel).unwrap_or(
		LevelFilter::from_str(default_loglevel)
			.expect("Could not parse loglevel. If you get this error, please Report!"),
	)
}

/// # Get LogLevel Env
///
#[cfg(feature = "from_custom_env")]
fn get_loglevel_custom_env(
	loglevel: &str,
	verbose: u64,
	quiet: u64,
	custom_env: &str,
) -> LevelFilter {
	get_loglevel(loglevel, verbose, quiet, vec![custom_env, "RUST_LOG"])
}
