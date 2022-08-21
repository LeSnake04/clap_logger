//! Module for creating and managing arguments for logging

pub use builder::ClapLogArgsBuilder;
pub use log_args::ClapLogArgs;

mod builder;
pub(crate) mod helper;
mod log_args;