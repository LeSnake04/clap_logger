use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use unwrap_or::{unwrap_ok_or, unwrap_some_or};

use crate::errors::{PolicyBuilderError as Error, PolicyBuilderResult as Result};

/// TODO: Doc
///
#[derive(Clone)]
pub struct PolicyBuilder {
	/// TODO: Doc
	size_trigger: Option<SizeTrigger>,
	/// TODO: Doc
	fixed_window: Option<FixedWindowRoller>,
}

impl Default for PolicyBuilder {
	fn default() -> Self {
		#[allow(clippy::expect_used)]
		Self::new()
			.size_limit(10)
			.window("", 5)
			.expect("Failed to set window. Please report!")
			.clone()
	}
}

impl PolicyBuilder {
	/// Create a new Policy builder
	pub fn new() -> Self {
		Self {
			size_trigger: None,
			fixed_window: None,
		}
	}

	/// TODO: Doc
	pub fn window(&mut self, prefix: &str, count: u32) -> Result<&mut Self> {
		self.fixed_window = Some(unwrap_ok_or!(
			FixedWindowRoller::builder().build(prefix, count),
			e,
			return Err(Error::FixedWindowRoller { source: e })
		));
		Ok(self)
	}

	/// TODO: Doc
	pub fn size_limit(&mut self, limit_kb: u64) -> &mut Self {
		self.size_trigger = Some(SizeTrigger::new(limit_kb * 1024));
		self
	}

	pub fn build(&mut self) -> Result<CompoundPolicy> {
		Ok(CompoundPolicy::new(
			Box::new(unwrap_some_or!(
				self.clone().size_trigger,
				return Err(Error::NoSizeTrigger)
			)),
			Box::new(unwrap_some_or!(
				self.clone().fixed_window,
				return Err(Error::NoFixedWindowRoller)
			)),
		))
	}
}
