# Clap Logger

Simple [fern][fern] integration for [clap][clap].

This crate provides a simple way to allow the user to set the log level via a command line argument.
It's directly implemented in clap, so it feels very naturally to use.

Please note this crate does not support `clap_derive` yet.

## Why Clap_logger ?

-  Multiple arguments:
   -  loglevel: to set the loglevel directly [loglevel = LevelFilter]
   -  verbose: increase loglevel (increases with each use)
   -  quiet: decrease loglevel (decreases with each use)
-  Argument can be safely modified
-  Loglevel via `RUST_LOG` or optionally custom Environment variables
-  directly embedded in [`clap::Command`] and [`clap::ArgMatches`]
-  no panics
-  Direct integration with `Clap::Command` and `Clap::ArgMatches`
-  Re-Exports basic clap and log items in order to allow you to minimize dependencies
-  just 2 extra lines for a basic implementation.
-  allows you to customize the log4rs config if you want to.
-  You can also just use this crate to get the loglevel and use you own logger.

## Roadmap

### 0.5

-  clap_derive support

## Initialising the logger

### Base implementation:

```rust
use clap::Command;
use log::LevelFilter;
use clap_logger::prelude::*;

fn main() {
 let m: clap::ArgMatches = Command::new("clap_command_test")
   // add the loglevel argument
   .add_logging_args()
	 .get_matches();

	 m.init_logger().expect("Failed to initialize logger");
}
```

## Crate Features

### Default

```toml
[dependencies]
clap_logger = "0.4"
```

-  cli arguments
-  console logger
-  get loglevel
-  prelude

### logfile

```toml
[dependencies]
clap_logger = {version="0.4", features=["logfile"]}
```

-  log to file

## Status: Beta

### Roadmap: 0.5

-  More tests
-  More examples.
-  `clap_derive` support

### Note:

1. If you get an error ending with `"Please report!"`, this happened very likely because of a bug in the library.
   Please report the error on [GitHub](https://github.com/LeSnake04/clap_logger/issues)