use std::path::Path;

use log::LevelFilter;
use log4rs::append::console::{ConsoleAppender, ConsoleAppenderBuilder};
use log4rs::append::file::{FileAppender, FileAppenderBuilder};
use log4rs::append::rolling_file::{RollingFileAppender, RollingFileAppenderBuilder};
use log4rs::append::Append;
use log4rs::config::runtime::{ConfigBuilder, RootBuilder};
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::{init_config, Config, Handle};

use crate::dbgm;
use crate::logger::policy_builder::PolicyBuilder;

/// TODO Doc
pub struct ClapLoggerBuilder {
	loglevel: LevelFilter,
	console: Option<ConsoleAppenderBuilder>,
	file: Option<FileLogger>,
	file_path: Option<&'static str>,
	file_config: Option<FileLoggerCfg>,
}

/// TODO Doc
#[derive(PartialEq, Debug, Clone)]
pub enum FileLogger {
	/// TODO Doc
	Continuous(FileAppenderBuilder),
	/// TODO Doc
	Rolling(RollingFileAppenderBuilder),
}

impl FileLogger {
	fn build(self, path: impl AsRef<Path>) -> Box<dyn Append> {
		match self {
			Self::Continuous(a) => Box::new(
				a.build(path)
					.expect("Continuous file appender failed to Build"),
			),
			Self::Rolling(a) => Box::new(
				a.build(path, Box::new(PolicyBuilder::default()))
					.expect("Rolling file appender failed to Build"),
			),
		}
	}
}

/// TODO Doc
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum FileLoggerCfg {
	/// TODO Doc
	Continuous,
	/// TODO Doc
	Rolling,
}

impl ClapLoggerBuilder {
	/// TODO Doc
	pub fn new(loglevel: LevelFilter) -> Self {
		Self {
			loglevel,
			console: None,
			file: None,
			file_path: None,
			file_config: None,
		}
	}
	#[cfg(feature = "console_logger")]
	/// TODO Doc
	pub fn custom_console_appender(&mut self, appender: ConsoleAppenderBuilder) -> &Self {
		self.console = Some(appender);
		self
	}

	#[cfg(feature = "console_logger")]
	/// TODO Doc
	pub fn console_logger(&mut self) -> &Self {
		if let Some(_) = self.console {
			panic!("clap_logger: LoggerBuilder: Console logger already added")
		}

		self.console = Some(
			ConsoleAppender::builder().encoder(Box::new(PatternEncoder::new(
				"{l} – {d(%Y-%M-%d – %H-%m-%S-%2F)} – {m}\n",
			))),
		);
		self
	}

	#[cfg(feature = "logfile")]
	/// TODO Doc
	pub fn custom_file_appender(&mut self, appender: FileLogger) -> &Self {
		self.file = Some(appender);
		self
	}

	#[cfg(feature = "logfile")]
	/// TODO Doc
	pub fn file_logger(&mut self, config: FileLoggerCfg) -> &Self {
		if let Some(_) = self.file {
			panic!("clap_logger: LoggerBuilder: Console logger already added")
		}

		self.file = Some(match config {
			FileLoggerCfg::Continuous => {
				FileLogger::Continuous(FileAppender::builder().encoder(Box::new(
					PatternEncoder::new("{l} – {d(%Y-%M-%d – %H-%m-%S-%2F)} – {m}\n"),
				)))
			}

			FileLoggerCfg::Rolling => {
				FileLogger::Rolling(RollingFileAppender::builder().encoder(Box::new(
					PatternEncoder::new("{l} – {d(%Y-%M-%d – %H-%m-%S-%2F)} – {m}\n"),
				)))
			}
		});
		self
	}
	/// TODO Doc
	pub fn init(&self) -> Handle {
		match (&self.file, &self.console) {
			(&None, &None) => panic!(
				"clap_logger: LoggerBuilder: you have to add at least one appender before .init()"
			),
			_ => (),
		}

		let mut config: ConfigBuilder = Config::builder();
		let mut root: RootBuilder = Root::builder();

		if let Some(a) = &self.console {
			config.appender(Appender::builder().build("stdout", Box::new(a.build())));
			root.appender("stdout");
		}

		let get_file_logger_cfg =
			|cfg_expected: FileLoggerCfg, expected: Option<FileLogger>| -> Option<ConfigBuilder> {
				if self.file_config == Some(cfg_expected) {
					if self.file == Some(expected) {
						Some(
							config.appender(
								Appender::builder().build(
									"stdout",
									Box::new(
										self.file
											.unwrap()
											.build(self.file_path.expect("No Path specified")),
									),
								),
							),
						)
					} else {
						None
					}
				} else {
					None
				}
			};

		let file_logger_continuous: Option<ConfigBuilder> =
			get_file_logger_cfg(FileLoggerCfg::Continuous, Some(FileLogger::Continuous()));

		let file_logger_rolling: Option<ConfigBuilder> =
			get_file_logger_cfg(FileLoggerCfg::Rolling);

		if (file_logger_rolling, file_logger_continuous) == (None, None) {
			root.appender("stdout");
		}

		init_config(
			config
				.build(root.build(self.loglevel))
				.expect("failed to initialize config"),
		)
		.expect("Failed to initialize Logger")
	}
}
