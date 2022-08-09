use clap::Arg;
use log::LevelFilter;

use crate::helper::{loglevel, loglevel_possible_values, quiet, verbose};

pub struct ClapLogArgsBuilder<'help> {
	loglevel: Arg<'help>,
	verbose: Arg<'help>,
	quiet: Arg<'help>,
	default_loglevel: LevelFilter,
}

impl<'help> ClapLogArgsBuilder<'help> {
	/// # New
	/// create ClapLogArgsBuilder with default Arguments.
	/// ## Arguments
	/// default_loglevel: Default Loglevel specified
	pub fn new(default_loglevel: LevelFilter) -> Self {
		Self {
			loglevel: loglevel(default_loglevel),
			verbose: verbose(),
			quiet: quiet(),
			default_loglevel,
		}
	}
	/// # Global
	/// Make loglevel args available to all sub
	pub fn global(mut self) -> Self {
		self.loglevel = self.loglevel.global(true);
		self.verbose = self.verbose.global(true);
		self.quiet = self.quiet.global(true);
		self
	}
	/// # Modify Loglevel Arg
	/// Modify the loglevel argument.
	///
	/// You will be given the current loglevel arg and can modify it however you want.
	/// ## Arguments
	/// arg: Closure taking [Arg][clap::Arg] and returning [Arg][clap::Arg];
	/// ## Limitations
	/// Changing these things would break functionality, so they will be reverted to default.
	/// + id = "loglevel"
	/// + possible_values = <available Loglevels>
	/// + default_loglevel = default_loglevel set previously
	pub fn modify_loglevel_arg(mut self, arg: fn(Arg) -> Arg) -> Self {
		self.loglevel = arg(self.loglevel)
			.id("loglevel")
			.default_value(self.default_loglevel.as_str())
			.possible_values(loglevel_possible_values());
		self
	}
	/// TODO: Doc
	pub fn change_default_loglevel(
		mut self,
		default_loglevel: LevelFilter,
	) -> ClapLogArgsBuilder<'help> {
		let default_loglevel_str: &'static str = default_loglevel.as_str();
		self.loglevel = self.loglevel.default_value(default_loglevel_str);
		self.default_loglevel = default_loglevel;
		self
	}

	/// # Modify the verbose argument.
	///
	/// You will be given the current loglevel arg and can modify it however you want.
	/// ## Arguments
	/// arg: Closure taking [Arg][clap::Arg] and returning [Arg][clap::Arg];
	/// ## Limitations
	/// Changing these things would break functionality, so they will be reverted to default.
	/// + id = `"verbose"`
	/// + takes_value = `false`
	/// + default_loglevel (will be the default_loglevel set previously)
	pub fn modify_verbose_arg(mut self, arg: fn(Arg) -> Arg) -> Self {
		self.verbose = arg(self.verbose)
			.id("verbose")
			.takes_value(false)
			.multiple_occurrences(true);
		self
	}
	/// # Modify the verbose argument.
	///
	/// You will be given the current loglevel arg and can modify it however you want.
	/// ## Arguments
	/// arg: Closure taking [Arg][clap::Arg] and returning [Arg][clap::Arg];
	/// ## Limitations
	/// Changing these things would break functionality, so they will be reverted to default.
	/// + id = `"verbose"`
	/// + takes_value = `false`
	/// + default_loglevel (will be the default_loglevel set previously)
	pub fn modify_quiet_arg(mut self, arg: fn(Arg) -> Arg) -> Self {
		self.quiet = arg(self.quiet)
			.id("quiet")
			.takes_value(false)
			.multiple_occurrences(true);
		self
	}
	/// # Heading
	/// Put logging Args in the "Debug" Category in the help overview.

	/// *if you want to specify the category name,
	/// use [`.custom_heading`][crate::ClapLogArgsBuilder::custom_heading]*
	pub fn heading(&mut self) -> &mut Self {
		self.loglevel = self.loglevel.clone().help_heading("Debug");
		self.verbose = self.verbose.clone().help_heading("Debug").clone();
		self.quiet = self.quiet.clone().help_heading("Debug").clone();
		self
	}
	/// # Custom Heading
	/// Put logging Args in the Specified Category in the help overview.
	///
	/// ## Arguments
	/// + heading: String or &str: Name of the Category
	pub fn custom_heading(&mut self, heading: impl AsRef<&'static str>) -> &mut Self {
		let heading: &str = heading.as_ref();
		self.loglevel = self.loglevel.clone().help_heading(heading);
		self.verbose = self.verbose.clone().help_heading(heading);
		self.quiet = self.quiet.clone().help_heading(heading);
		self
	}
	/// # Export
	/// Export Arguments as Array of [Args][clap::Arg]
	pub fn export(self) -> [Arg<'help>; 3] {
		[self.loglevel, self.verbose, self.quiet]
	}
}
