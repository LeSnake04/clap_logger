use anyhow::Result;
use std::env;

use crate::prelude::*;

#[test]
fn env_loglevel_invalid() -> Result<()> {
	env::set_var("TEST_LOGLEVEL", "abc");

	let m: ArgMatches = Command::new("clap_command_test")
		.arg(arg!(-a --alpha "bla"))
		.add_logging_args(LevelFilter::Info)
		.get_matches_from(["clap_logger", "--loglevel", "OFF"]);

	m.init_logger_custom_env(EnvLogLevelHandling::OverwriteDefault(
		"TEST_LOGLEVEL".to_string(),
	))
	.cause("Failed to init logger");

	trace!("trace");
	debug!("debug");
	info!("info");
	warn!("warn");
	error!("error");
	Ok(())
}
