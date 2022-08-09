use anyhow::{Context, Result};

use crate::prelude::*;

#[test]
fn main() -> Result<()> {
	let mut cmd: Command = Command::new("clap_command_test")
		.arg(arg!(-a --alpha "bla"))
		.add_logging_args(LevelFilter::Info);

	cmd.print_long_help().context("Failed to print help")?;

	let m: ArgMatches = cmd
		.clone()
		.get_matches_from(["clap_logger", "--loglevel", "trace"]);
	m.init_logger().context("Failed to init logger")?;

	trace!("trace");
	debug!("debug");
	info!("info");
	warn!("warn");
	error!("error");
	Ok(())
}
