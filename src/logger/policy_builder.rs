use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;

/// TODO Doc
pub struct PolicyBuilder {
	/// TODO Doc
	size_trigger: SizeTrigger,
	/// TODO Doc
	fixed_window: FixedWindowRoller,
}

impl PolicyBuilder {
	pub fn default() -> &'static Self {
		Self.size_limit(5).windows()
	}

	/// TODO Doc
	pub fn windows(&mut self, prefix: &str, count: u32) -> &Self {
		self.fixed_window = FixedWindowRoller::builder()
			.build(&format!("{}", prefix), count)
			.expect("Failed to build FixedWindowRoller, please report!");
		self
	}

	/// TODO Doc
	pub fn size_limit(&mut self, limit_kb: u64) -> &Self {
		self.size_trigger = SizeTrigger::new(limit_kb * 1024);
		self
	}

	pub fn compound(&mut self) -> CompoundPolicy {
		CompoundPolicy::new(Box::new(&self.size_trigger), Box::new(&self.fixed_window))
	}
}
