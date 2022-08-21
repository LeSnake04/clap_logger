use clap::Arg;
use log::LevelFilter;

use crate::helper::{loglevel, loglevel_possible_values, quiet, verbose};

use super::helper::loglevel_file;

/// TODO: Doc
pub struct ClapLogArgsBuilder<'help> {
	loglevel: Arg<'help>,
	verbose: Arg<'help>,
	quiet: Arg<'help>,
	loglevel_file: Option<Arg<'help>>,
	default_loglevel: LevelFilter,
	default_loglevel_file: Option<LevelFilter>,
}

impl<'help> ClapLogArgsBuilder<'help> {
	/// # New
	/// create ClapLogArgsBuilder with default Arguments.
	/// ## Arguments
	/// default_loglevel: LevelFilter = Default Loglevel
	pub fn new(default_loglevel: LevelFilter) -> Self {
		Self {
			loglevel: loglevel(default_loglevel),
			verbose: verbose(),
			quiet: quiet(),
			default_loglevel,
			loglevel_file: None,
			default_loglevel_file: None,
		}
	}
	/// TODO: Doc
	pub fn file_logger(mut self, default_loglevel_file: LevelFilter) -> Self {
		self.default_loglevel_file = Some(default_loglevel_file);
		self.loglevel_file = Some(loglevel_file(default_loglevel_file));
		self
	}

	/// # Global
	/// Make loglevel args available to all sub
	pub fn global(mut self, yes: bool) -> Self {
		self.loglevel = self.loglevel.global(yes);
		self.verbose = self.verbose.global(yes);
		self.quiet = self.quiet.global(yes);
		if let Some(a) = self.loglevel_file {
			self.loglevel_file = Some(a.global(yes))
		}
		self
	}

	/// # Set the default loglevel
	/// ## Arguments
	/// default_loglevel: LevelFilter = New default loglevel
	pub fn change_default_loglevel(
		mut self,
		default_loglevel: LevelFilter,
	) -> ClapLogArgsBuilder<'help> {
		self.loglevel = self.loglevel.default_value(default_loglevel.as_str());
		self.default_loglevel = default_loglevel;
		self
	}

	/// TODO: Doc
	pub fn change_default_loglevel_file(
		mut self,
		default_loglevel_file: LevelFilter,
	) -> ClapLogArgsBuilder<'help> {
		self.loglevel_file = Some(
			self.loglevel
				.clone()
				.default_value(default_loglevel_file.as_str()),
		);
		self.default_loglevel_file = Some(default_loglevel_file);
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
	pub fn modify_loglevel_file_arg(mut self, arg: fn(Arg) -> Arg) -> Self {
		self.loglevel_file = Some(
			arg(self.loglevel.clone())
				.id("loglevel")
				.default_value(self.default_loglevel.as_str())
				.possible_values(loglevel_possible_values()),
		);
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
	///
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
	/// Export Arguments as Vector of [Args][clap::Arg]
	pub fn export(self) -> Vec<Arg<'help>> {
		let mut out: Vec<Arg> = vec![self.loglevel, self.verbose, self.quiet];
		if let Some(a) = self.loglevel_file {
			out.push(a);
		}
		out
	}
}