use log::ParseLevelError;
use snafu::Snafu;

use crate::errors::clap_logger_builder::ClapLoggerBuilderError;

/// Result returned by [ClapInitLogger][crate::ClapInitLogger]
pub type ClapInitLoggerResult<T> = Result<T, ClapInitLoggerError>;

/// Errors occoring in [ClapInitLogger][crate::ClapInitLogger]
#[derive(Snafu, Debug)]
pub enum ClapInitLoggerError {
	#[snafu(display("Failed to Build Logger: {source}"))]
	BuilderError { source: ClapLoggerBuilderError },
	#[snafu(display("Could not find loglevel argument. Please make sure you added the Command"))]
	CouldntFindLoglevelArg,
	#[snafu(display("Could not parse loglevel. If you get this error, please Report!: {source}"))]
	CouldntParseLoglevel { source: ParseLevelError },
	#[snafu(display("Could not parse loglevel. If you get this error, please Report!"))]
	InvalidLoglevelIndex { index: usize },
	#[snafu(display("Could not get loglevel index. If you get this error, please Report!"))]
	CouldntGetLoglevelIndex,
}

impl From<ClapLoggerBuilderError> for ClapInitLoggerError {
	fn from(input: ClapLoggerBuilderError) -> Self {
		Self::BuilderError { source: input }
	}
}