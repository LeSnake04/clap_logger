# Logging Arguments

This modules provides the Logging arguments

## Adding the Argument

### Base Implementation:

```
use clap_logger::prelude::*;

// Generate a clap command
let m: ArgMatches = Command::new("clap_command_test")//!
  // add logging arguments
	  .add_logging_args()
	 .get_matches();
```

## loglevel Arg manipulation

You can also get the [Arg][clap::arg] directly in order to modify it before adding:`

```
use clap_logger::prelude::*;

// Generate a clap command TODO: Update example
let m: ArgMatches = Command::new("clap_command_test")
  // add the add loglevel argument
	  .arg(get_loglevel_arg(LevelFilter::Debug)
		 // Adding a short version
		 .short('l')
	 // changing the long version of the argument
		 .long("log")
	 // make it required to annoy the user
	 .required(true))
	 .get_matches();
```

Warning: Do NOT touch the [`.possible_values`], [`.id`] or field of the argument or anything in that modifies the user input.