use crate::prelude::*;
use std::env;

#[test]
#[cfg(feature = "from_custom_env")]
fn main() {
	env::set_var("TEST_LOGLEVEL", "");

	let m: ArgMatches = Command::new("clap_command_test")
		.arg(arg!(-a --alpha "bla"))
		.add_logging_args(LevelFilter::Info)
		.get_matches_from(["clap_logger", "--loglevel", "OFF"]);

	m.init_logger_custom_env(EnvLogLevelHandling::OverwriteDefault(
		"TEST_LOGLEVEL".to_string(),
	))
	.expect("Failed to initialize logger");

	trace!("trace");
	debug!("debug");
	info!("info");
	warn!("warn");
	error!("error");
}
