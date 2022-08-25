use std::io::stdout;

use fern::{log_file, Dispatch};
use log::LevelFilter;

use crate::errors::{ClapLoggerBuilderError as Error, ClapLoggerBuilderResult as Result};

/// # Logger Builder
/// Configure the logger
///
/// TODO: example
pub struct ClapLoggerBuilder {
	loglevel: LevelFilter,
	root: Dispatch,
	custom_format: bool,
	console: bool,
	console_custom: Option<Dispatch>,
	loglevel_file: Option<LevelFilter>,
	file: bool,
	file_custom: Option<Dispatch>,
	file_path: Option<String>,
}

/// # File Logger Type
/// Defines the type of the logfile
/// + Continuous
///
/// Always attach message to the Same file
/// + Rolling
///
/// Roll files on certain conditions
impl ClapLoggerBuilder {
	/// TODO: Docbuilder
	pub fn new(loglevel: LevelFilter) -> Self {
		let root: Dispatch = Dispatch::new();
		Self {
			loglevel,
			loglevel_file: None,
			root,
			custom_format: false,
			console: true,
			console_custom: None,
			file: false,
			file_custom: None,
			file_path: None,
		}
	}

	#[cfg(feature = "console_logger")]
	/// # Custom Console Appender
	///
	/// *For advanced users only!*
	///
	/// Manually set the [Dispatch][fern::Dispatch] applied to file and console
	/// dispath.
	///
	/// Please note the loglevel will be reset to the default loglevel.
	///
	/// ## Arguments
	/// + dispatch: Dispatch - Dispatch used as root
	/// + custom_format: bool - Set to ´true´ if you set a custom `.format()`, so the builder won't
	/// overwrite it
	pub fn custom_root(mut self, custom_format: bool, dispatch: Dispatch) -> Self {
		self.custom_format = custom_format;
		self.root = dispatch;
		self
	}

	#[cfg(feature = "console_logger")]
	/// # Custom Console Appender
	/// *For advanced users only!*
	///
	/// Manually set the [Dispatch][fern::Dispatch] used for the console logger.
	///
	/// This will be chained into the root
	///
	/// Please note the loglevel will be reset to the default loglevel.
	pub fn custom_console_appender(&mut self, dispatch: Dispatch) -> &Self {
		self.console_custom = Some(dispatch.level(self.loglevel));
		self
	}

	#[cfg(feature = "console_logger")]
	/// TODO: Doc
	pub fn console_logger(mut self, yes: bool) -> Self {
		self.console = yes;
		self
	}

	#[cfg(feature = "logfile")]
	/// # Custom File Dispatch
	/// For advanced users only!*
	/// TODO: Update Doc
	/// Manually set the [Dispatch][fern::Dispatch]
	/// ## Arguments
	/// + appender: Dispatch
	/// *You Specify a FileAppenderBuilder so the the path can be specified like normal.*
	pub fn custom_file_appender(&mut self, dispatch: Dispatch) -> Result<&Self> {
		self.file_custom = Some(dispatch);
		Ok(self)
	}

	#[cfg(feature = "logfile")]
	/// TODO: Doc
	pub fn file_logger(&mut self, yes: bool) -> &mut Self {
		self.file = yes;
		self
	}

	#[doc(hidden)]
	fn get_logger_builder(&self) -> Result<(Option<Dispatch>, Option<Dispatch>)> {
		Ok((
			self.console
				.then_some(Dispatch::new().level(self.loglevel).chain(stdout())),
			match self.file {
				false => None,
				true => Some(
					Dispatch::new()
						.level(self.loglevel_file.unwrap_or(self.loglevel))
						.chain(
							log_file(self.file_path.clone().ok_or(Error::FilePathNotSet)?)
								.map_err(|e| Error::LogFileErrror { source: e })?,
						),
				),
			},
		))
	}

	#[cfg(feature = "init_logger")]
	pub(crate) fn init(self) -> Result<()> {
		match (
			match self.get_logger_builder()? {
				(None, None) => return Err(Error::NoAppenderGiven),
				(Some(c), None) => self.root.chain(c),
				(None, Some(f)) => self.root.chain(f),
				(Some(c), Some(f)) => self.root.chain(c).chain(f),
			},
			self.custom_format,
		) {
			(a, true) => a,
			(a, false) => a.format(|out, message, record| {
				out.finish(format_args!(
					"{}[{}][{}] {}",
					chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
					record.target(),
					record.level(),
					message
				))
			}),
		}
		.apply()
		.map_err(|e| Error::InitFailed { source: e })
	}
}
