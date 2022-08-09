#![allow(unused_imports)]
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::{FileAppender, FileAppenderBuilder};
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::Policy;
use log4rs::append::rolling_file::{RollingFileAppender, RollingFileAppenderBuilder};
use log4rs::config::runtime::{ConfigBuilder, LoggerBuilder, RootBuilder};
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::{init_config, Config, Handle};
use unwrap_or::unwrap_some_or;

use crate::errors::{ClapLoggerBuilderError as Error, ClapLoggerBuilderResult as Result};
use crate::PolicyBuilder;

/**
# Logger Builder
Configure the logger

TODO: example
*/
pub struct ClapLoggerBuilder {
	loglevel: LevelFilter,
	console: Option<ConsoleAppender>,
	file: Option<FileAppenderBuilder>,
	file_rolling: Option<RollingFileAppenderBuilder>,
	policy: Option<CompoundPolicy>,
	file_path: Option<String>,
	file_type: Option<FileLoggerType>,
}

/**
# File Logger Type
Defines the type of the logfile
+ Continuous

Always attach message to the Same file
+ Rolling

Roll files on certain conditions
*/
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum FileLoggerType {
	/// Always attach message to the Same file
	Continuous,
	/// Roll files on certain conditions
	Rolling,
}

impl ClapLoggerBuilder {
	/// TODO: Docbuilder
	pub fn new(loglevel: LevelFilter) -> Self {
		Self {
			loglevel,
			console: None,
			file: None,
			file_path: None,
			file_type: None,
			policy: None,
			file_rolling: None,
		}
	}

	#[cfg(feature = "console_logger")]
	/// # Custom Console Appender
	/// *For advanced users only!*
	/// Manually set the [ConsoleAppender][log4rs::append::console::ConsoleAppender]
	pub fn custom_console_appender(&mut self, appender: ConsoleAppender) -> &Self {
		self.console = Some(appender);
		self
	}

	#[cfg(feature = "console_logger")]
	/// TODO: Doc
	pub fn add_console_logger(&mut self) -> Result<&mut Self> {
		if self.console.is_some() {
			return Err(Error::ConsoleAppenderAlreadyExists);
		}

		self.console = Some(
			ConsoleAppender::builder()
				.encoder(Box::new(PatternEncoder::new(
					"{l} – {d(%Y-%M-%d – %H-%m-%S-%2F)} – {m}\n",
				)))
				.build(),
		);
		Ok(self)
	}

	#[cfg(feature = "logfile")]
	/// # Custom Rolling File Appender
	/// For advanced users only!*

	/// Manually set the [FileAppender][log4rs::append::file::FileAppender]
	/// ## Inputs
	/// + appender: [FileAppenderBuilder][log4rs::append::file::FileAppender]
	/// *You Specify a FileAppenderBuilder so the the path can be specified like normal.*
	pub fn custom_file_appender(&mut self, appender: FileAppenderBuilder) -> Result<&Self> {
		/*if self.file_rolling.is_some() {
		return Err(Error::FileAppenderAlreadyExists);
		}*/
		self.file_type = Some(FileLoggerType::Continuous);
		self.file = Some(appender);
		Ok(self)
	}

	/// # Custom Rolling File Appender
	/// *For advanced users only!*
	///
	/// Manually set the [RollingFileAppender][log4rs::append::rolling_file::RollingFileAppender]
	/// ## Arguments
	/// + [appender][log4rs::append::rolling_file::RollingFileAppender]: You Specify a RollingFileAppenderBuilder so the the path can be specified like normal.
	pub fn custom_rolling_file_appender(
		&mut self,
		appender: RollingFileAppenderBuilder,
	) -> Result<&Self> {
		/*if self.file.is_some() {
		return Err(Error::FileAppenderAlreadyExists);
		}*/
		self.file_type = Some(FileLoggerType::Rolling);
		self.file_rolling = Some(appender);
		Ok(self)
	}

	#[cfg(feature = "logfile")]
	/// TODO: Doc
	pub fn file_logger(&mut self, logger_type: FileLoggerType) -> Result<&mut Self> {
		if self.file.is_some() {
			return Err(Error::CantUseRollingAndContinuous);
		}

		fn rolling_file() -> Result<RollingFileAppenderBuilder> {
			todo!("TODO: Write RollingFileAppender")
		}

		fn file() -> Result<FileAppenderBuilder> {
			todo!("TODO: Write FileAppender")
		}

		match logger_type {
			FileLoggerType::Continuous => {
				self.file = Some(file()?);
				self.file_type = Some(FileLoggerType::Continuous);
			}
			FileLoggerType::Rolling => {
				self.file_rolling = Some(rolling_file()?);
				self.file_type = Some(FileLoggerType::Continuous);
			}
		}
		Ok(self)
	}
	pub(crate) fn init(&self) -> Result<Handle> {
		if self.file_type.is_none() && self.console.is_none() {
			return Err(Error::NoAppenderGiven);
		}

		#[allow(unused_mut)]
		let mut config: ConfigBuilder = Config::builder();
		#[allow(unused_mut)]
		let mut root: RootBuilder = Root::builder();

		if self.console.is_some() {
			// FIXME: Why TF are trait bounds not satisfied??
			&config.appender(
				Appender::builder().build("stdout", Box::new(&self.clone().console.unwrap())),
			);
			&root.appender("stdout");
		}

		#[allow(unused)]
		if let Some(f) = &self.file_type {
			// FIXME: Why TF are trait bounds not satisfied??
			match f {
				FileLoggerType::Continuous => &config.appender(Appender::builder().build(
					"stderr",
					Box::new(self.file.ok_or(Error::FileAppenderNotFound)?),
				)),
				FileLoggerType::Rolling => &config.appender(
					Appender::builder().build(
						"stderr",
						Box::new(
							self.file_rolling
								.map(|a| {
									a.build(
										self.file_path.ok_or(Error::FilePathNotFound)?,
										self.policy.ok_or(Error::NoPolicyGiven)?,
									)
								})
								.ok_or(Error::FileAppenderNotFound)?,
						),
					),
				),
			};
			&root.appender("stderr");
		}
		init_config(
			config
				.build(root.build(self.loglevel))
				.map_err(|e| Error::BuildFailed { source: e })?,
		)
		.map_err(|e| Error::InitFailed { source: e })
	}
}