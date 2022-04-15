use crate::{ClapInitLogger, ClapLoglevelArg};
use clap::{arg, ArgMatches, Command};
use log::{debug, error, info, trace, warn, LevelFilter};
use std::env;

/*#[derive(Parser)]
struct ClapDerive {
	// TODO find a way to add integrate with clap parse
}*/

/*#[test]
fn clap_derive() {
	// TODO parse clap derive example
}*/

#[test]
fn clap_command() {
	let m: ArgMatches = Command::new("clap_command_test")
		.arg(arg!(-a --alpha "bla"))
		.add_loglevel_arg(LevelFilter::Info)
		.get_matches_from(["clap_logger", "--loglevel", "OFF "]);
	m.init_logger();
	trace!("trace");
	debug!("debug");
	info!("info");
	warn!("warn");
	error!("error");
}

#[test]
fn env_loglevel_invalid() {
	env::set_var("TEST_LOGLEVEL", "abc");

	let m: ArgMatches = Command::new("clap_command_test")
		.arg(arg!(-a --alpha "bla"))
		.add_loglevel_arg(LevelFilter::Info)
		.get_matches_from(["clap_logger", "--loglevel", "OFF "]);
	m.init_logger();
	trace!("trace");
	debug!("debug");
	info!("info");
	warn!("warn");
	error!("error");
}
