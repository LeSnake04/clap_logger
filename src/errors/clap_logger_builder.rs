//! Errors occurring in [LoggerBuilder][crate::logger::builder]

use log::SetLoggerError;
use log4rs::config::runtime::ConfigErrors;
use snafu::Snafu;

pub type ClapLoggerBuilderResult<T> = Result<T, ClapLoggerBuilderError>;

#[derive(Snafu, Debug)]
pub enum ClapLoggerBuilderError {
	#[snafu(display("No console appender given. Please Report!"))]
	NoConsoleAppenderGiven,
	#[snafu(display("No Appender. Please add at least one appender"))]
	NoAppenderGiven,
	#[snafu(display("No Compound policy given. Please Specify a CompundPolicy"))]
	NoPolicyGiven,
	#[snafu(display("File appender already exists."))]
	FileAppenderAlreadyExists,
	#[snafu(display("You cant have both rolling and continuous file appender."))]
	CantUseRollingAndContinuous,
	#[snafu(display("Console Appender already set"))]
	ConsoleAppenderAlreadyExists,
	#[snafu(display("Could not find File appender based on Type given. Please Report!"))]
	FileAppenderNotFound,
	#[snafu(display("File Path not set. Please Report!"))]
	FilePathNotFound,
	#[snafu(display("Logger init failed: {source}"))]
	InitFailed {
		#[snafu(source)]
		source: SetLoggerError,
	},
	#[snafu(display("Logger build failed: {source}"))]
	BuildFailed {
		#[snafu(source)]
		source: ConfigErrors,
	},
}