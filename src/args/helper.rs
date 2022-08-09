use clap::{Arg, PossibleValue};
use log::LevelFilter;

#[doc(hidden)]
pub(crate) fn loglevel_possible_values<'a>() -> [PossibleValue<'a>; 6] {
	[
		PossibleValue::new("OFF").help("Disable Logging completely"),
		PossibleValue::new("ERROR").help("Only show Error messages"),
		PossibleValue::new("WARN").help("Only show Warnings and Errors"),
		PossibleValue::new("INFO").help("Show Information, Warnings and Errors"),
		PossibleValue::new("DEBUG").help("Show Debug information and upward messages"),
		PossibleValue::new("TRACE").help("Show all messages"),
	]
}

pub fn loglevel<'a>(default_loglevel: LevelFilter) -> Arg<'a> {
	Arg::new("loglevel")
		.long("loglevel")
		.required(false)
		.default_value(default_loglevel.as_str())
		.help("Set the loglevel")
		.long_help("Set the loglevel. TRACE is the most verbose and OFF the least verbose")
		.possible_values(loglevel_possible_values())
}

#[doc(hidden)]
pub(crate) static LEVEL_FILTERS: [&str; 6] = ["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

#[doc(hidden)]
#[allow(unused)]
pub(crate) fn get_loglevel_difference(default_loglevel: LevelFilter) -> (usize, usize) {
	let default_loglevel_pos: usize = LEVEL_FILTERS
		.binary_search(&default_loglevel.as_str())
		.unwrap_or({
			println!("clap_logger: Failed to get position of default loglevel, using Warn. Please Report!(This message will be hidden in release builds)");
			2
		})
		.clamp(0, 5);

	(
		LEVEL_FILTERS
			.len()
			.clamp(0, 5)
			.saturating_sub(default_loglevel_pos),
		default_loglevel_pos.saturating_sub(1),
	)
}

#[doc(hidden)]
pub(crate) fn get_loglevel_index(default_loglevel: LevelFilter) -> usize {
	LEVEL_FILTERS
		.binary_search(&default_loglevel.as_str())
		.unwrap_or({
			println!("Verbose/Silent: Failed to get position of default loglevel, using Warn");
			2
		})
		.clamp(0, 5)
}

#[doc(hidden)]
pub fn verbose<'a>() -> Arg<'a> {
	Arg::new("verbose")
		.short('v')
		.long("verbose")
		.help("Increase verbosity, increases for each use")
		.multiple_occurrences(true)
}

#[doc(hidden)]
pub fn quiet<'a>() -> Arg<'a> {
	Arg::new("quiet")
		.short('q')
		.long("quiet")
		.help("Decrease verbosity, decreases for each use")
		.multiple_occurrences(true)
}
