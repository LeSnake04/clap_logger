#![cfg(feature = "custom_env")]
use anyhow::{Context, Result};
use std::env;

use clap_logger::prelude::*;

#[allow(dead_code)]
fn main() -> Result<()> {
	env::set_var("TEST_LOGLEVEL", "abc");

	let m: ArgMatches = Command::new("clap_command_test")
		.arg(arg!(-a --alpha "bla"))
		.add_logging_args(LevelFilter::Info)
		.get_matches_from(["clap_logger", "--loglevel", "OFF"]);

	m.init_logger_custom_env("TEST_LOGLEVEL")
		.context("Failed to init logger")?;

	trace!("trace");
	debug!("debug");
	info!("info");
	warn!("warn");
	error!("error");
	Ok(())
}