use snafu::Snafu;

/// Result returned by [ClapLogArgs][crate::ClapLogArgs]
pub type ClapLogArgsResult<T> = Result<T, ClapLogArgsError>;

/// Errors occoring in [ClapLogArgs][crate::ClapLogArgs]
#[derive(Snafu, Debug)]
pub enum ClapLogArgsError {
	#[snafu(display("Logging Arguments are missing. Please make sure .add_logging_args(...) or .add_modified_logging_args(...) is called."))]
	MissingArguments,
	#[snafu(display("Arguments found multiple times. Make sure you only called .add_logging_args(...) OR .add_modified_logging_args(...) once"))]
	MultipleArguments,
}
