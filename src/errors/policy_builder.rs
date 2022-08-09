use std::fmt::Debug;

use snafu::Snafu;

pub type PolicyBuilderResult<T> = Result<T, PolicyBuilderError>;

#[derive(Snafu, Debug)]
pub enum PolicyBuilderError {
	#[snafu(display("Failed to build FixedWindowRoller: {source}"))]
	FixedWindowRoller { source: anyhow::Error },
	#[snafu(display("No SizeTrigger set"))]
	NoSizeTrigger,

	#[snafu(display("No FixedWindowRoller set"))]
	NoFixedWindowRoller,
}
