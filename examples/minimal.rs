use clap_logger::{ClapInitLogger, ClapLogArgs};
use log4rs::Handle;

fn main() {
	use clap_logger::prelude::*;

	// Generate a clap command
	let m: ArgMatches = Command::new("clap_command_test")
		.arg(arg!(-a --alpha "hello world!"))
		.arg(
			Arg::new("input")
				.short('i')
				.long("input")
				.takes_value(true)
				.required(false),
		)
		// add the logging arguments
		.add_logging_args(LevelFilter::Trace)
		.get_matches();

	m.init_logger();

	error!("Error");
	warn!("Warn");
	info!("Info");
	debug!("Debug");
	trace!("Trace")
}
