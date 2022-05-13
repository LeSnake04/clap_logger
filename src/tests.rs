#[cfg(feature = "from_custom_env")]
use std::env;

use crate::prelude::*;

// TODO find a way to add integrate with clap parse
/*#[derive(Parser)]
struct ClapDerive {

}*/

// TODO parse clap derive example
/*#[test]
fn clap_derive() {

}*/

#[test]
fn clap_command() {
	let cmd: Command = Command::new("clap_command_test")
		.arg(arg!(-a --alpha "bla"))
		.add_logging_args(LevelFilter::Info);

	cmd.clone().print_long_help();

	let m: ArgMatches = cmd
		.clone()
		.get_matches_from(["clap_logger", "--loglevel", "trace"]);
	m.init_logger();

	trace!("trace");
	debug!("debug");
	info!("info");
	warn!("warn");
	error!("error");
}

#[test]
#[cfg(feature = "from_custom_env")]
fn env_loglevel_invalid() {
	env::set_var("TEST_LOGLEVEL", "abc");

	let m: ArgMatches = Command::new("clap_command_test")
		.arg(arg!(-a --alpha "bla"))
		.add_logging_args(LevelFilter::Info)
		.get_matches_from(["clap_logger", "--loglevel", "OFF"]);

	m.init_logger_custom_env(EnvLogLevelHandling::OverwriteDefault(
		"TEST_LOGLEVEL".to_string(),
	));

	trace!("trace");
	debug!("debug");
	info!("info");
	warn!("warn");
	error!("error");
}

#[test]
#[cfg(feature = "from_custom_env")]
fn env_loglevel_valid() {
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

#[test]
fn verbose_overuse() {
	let m: ArgMatches = Command::new("clap_command_test")
		.arg(arg!(-a --alpha "bla"))
		.add_logging_args(LevelFilter::Warn)
		.get_matches_from(["clap_logger", "-vvvvvvvvvvv"]);

	m.init_logger();

	trace!("trace");
	debug!("debug");
	info!("info");
	warn!("warn");
	error!("error");
}
