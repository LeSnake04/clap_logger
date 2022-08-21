use log::SetLoggerError;
use snafu::Snafu;
use std::io::Error as IoError;
/// Result returned in [LoggerBuilder][crate::logger::builder]
pub type ClapLoggerBuilderResult<T> = Result<T, ClapLoggerBuilderError>;

/// Errors occurring in [LoggerBuilder][crate::logger::builder]
#[derive(Snafu, Debug)]
pub enum ClapLoggerBuilderError {
	#[snafu(display("No console appender given. Please Report!"))]
	NoConsoleAppenderGiven,
	#[snafu(display("No Appender. Please add at least one appender"))]
	NoAppenderGiven,
	#[snafu(display("No Compound policy given. Please Specify a CompundPolicy"))]
	NoPolicyGiven,
	#[snafu(display("Could not find File appender based on Type given. Please Report!"))]
	FileAppenderNotFound,
	#[snafu(display("File Path not set. Please Report!"))]
	FilePathNotSet,
	#[snafu(display("Failed to create logfile: "))]
	LogFileErrror { source: IoError },
	#[snafu(display("Failed to initialiaze logge because {source}"))]
	InitFailed { source: SetLoggerError },
}