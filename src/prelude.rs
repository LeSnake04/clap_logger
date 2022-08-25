/*!
```
use clap_logger::prelude::*;
```
# Collection of imports for setting up the crate.
Also re-exports clap and log commands needed for implementation to reduce imports and dependencies.
[See start page for implementation details.][crate]
Includes
- essential modules for setting up clap_logger
- needed types for type specifications
- basic clap modules (like [ArgMatches][clap::ArgMatches],[Command][clap::Command] and many more)
- logging functions and LevelFilters
 */
pub use clap::{arg, command, Arg, ArgMatches, Command};
pub use fern::Dispatch;
pub use log::LevelFilter;

pub use crate::log::*;
pub use crate::ClapInitLogger;
pub use crate::ClapLogArgs;
