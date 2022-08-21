use anyhow::{Context, Result};
use clap_logger::prelude::*;

#[allow(dead_code)]
fn main() -> Result<()> {
	let m: ArgMatches = Command::new("clap_command_test")
		.arg(arg!(-a --alpha "bla"))
		.add_logging_args(LevelFilter::Warn)
		.get_matches_from(["clap_logger", "-vvvvvvvvvvv"]);

	m.init_logger().context("Failed to initialize logger")?;
	assert_eq!(m.get_loglevel()?, LevelFilter::Trace);
	Ok(())
}