use crate::prelude::*;
use anyhow::{Context, Result};

#[test]
fn main() -> Result<()> {
	let m: ArgMatches = Command::new("clap_command_test")
		.arg(arg!(-a --alpha "bla"))
		.add_logging_args(LevelFilter::Warn)
		.get_matches_from(["clap_logger", "-vvvvvvvvvvv"]);

	m.init_logger().context("Failed to initialize logger")?;

	trace!("trace");
	debug!("debug");
	info!("info");
	warn!("warn");
	error!("error");
	Ok(())
}
