use anyhow::{Context, Result};
use clap_logger::prelude::*;

#[allow(dead_code)]
fn main() -> Result<()> {
	let m: ArgMatches = Command::new("clap_command_test")
		.add_logging_args(LevelFilter::Warn)
		.get_matches_from(["clap_logger", "-qqqqqqqqqqq"]);

	m.init_logger().context("Failed to initialize logger")?;
	assert_eq!(m.get_loglevel()?, LevelFilter::Off);
	Ok(())
}