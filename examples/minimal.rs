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
		// add the loglevel argument
		.add_loglevel_arg(LevelFilter::Off)
		.get_matches();

	m.init_env_logger().expect("Failed to initialize logger");
}
