#![doc(issue_tracker_base_url = "https://github.com/lesnake04/clap_logger/issues/")]
#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(missing_docs)]
#![warn(rustdoc::all)]
#![warn(clippy::indexing_slicing)]
#![allow(rustdoc::private_doc_tests)]
#![doc = include_str!("lib.md")]


pub(crate) use crate::args::helper;
pub use crate::args::{ClapLogArgs, ClapLogArgsBuilder};
pub use crate::logger::{ClapInitLogger, ClapLoggerBuilder};


pub mod args;
pub mod errors;
#[cfg(feature = "prelude")]
pub mod log;
#[cfg(feature = "init_logger")]
pub mod logger;
#[cfg(feature = "prelude")]
pub mod prelude;