use log::ParseLevelError;
use snafu::Snafu;

use crate::errors::clap_logger_builder::ClapLoggerBuilderError;

pub type ClapInitLoggerResult<T> = Result<T, ClapInitLoggerError>;
#[derive(Snafu, Debug)]
pub enum ClapInitLoggerError {
	#[snafu(display("Failed to Build Logger: {source}"))]
	BuilderError { source: ClapLoggerBuilderError },
	#[snafu(display("Could not find loglevel argument. Please make sure you added the Command"))]
	CouldntFindLoglevelArg,
	#[snafu(display("Could not parse loglevel. If you get this error, please Report!: {source}"))]
	CouldntParseLoglevel { source: ParseLevelError },
}

impl From<ClapLoggerBuilderError> for ClapInitLoggerError {
	fn from(input: ClapLoggerBuilderError) -> Self {
		Self::BuilderError { source: input }
	}
}
