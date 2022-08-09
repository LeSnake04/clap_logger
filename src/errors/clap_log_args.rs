use snafu::Snafu;

pub type ClapLogArgsResult<T> = Result<T, ClapLogArgsError>;

#[derive(Snafu, Debug)]
pub enum ClapLogArgsError {
	#[snafu(display("Logging Arguments are missing. Please make sure .add_logging_args(...) or .add_modified_logging_args(...) is called."))]
	MissingArguments,
	#[snafu(display("Arguments found multiple times. Make sure you only called .add_logging_args(...) OR .add_modified_logging_args(...) once"))]
	MultipleArguments,
}
