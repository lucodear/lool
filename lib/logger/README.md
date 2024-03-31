<p align="center"><img src="./../../.github/img/logo-logger.svg" height="192"></p>

<br>
<br>
<br>

<p align="center"><b>lool » <code>logger</code></b> implements a basic console logger to use with the <a href="https://crates.io/crates/log"><code>log</code></a> crate.
</p>

<br>
<br>
<br>

# Installation

This crate is for internal use. It's only published privately. 

```bash
cargo add lool --registry=lugit
```

# Setup

The logger must be initialized before it can be used. The following code snippet shows how to initialize the logger with the default settings:

```rs
use lool::logger::{ConsoleLogger, Level};

fn main() {
    ConsoleLogger::default_setup(Level::Trace, "my-app").unwrap();
}
```

The `default_setup` function takes two arguments:

- The maximum log level to display.
- The name of the logger (usually, the name of the application).


# Usage

The logger can be used with the `log` crate. The following code snippet shows how to use the logger:

```rs
use lool::logger::{info, warn, error, debug, trace};

fn main() {
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");
    debug!("This is a debug message");
    trace!("This is a trace message");
}
```

Which will result in the following output:

```log
[my-app] 2024-03-31 15:44:46 | INFO  | main.rs:4 - This is an info message
[my-app] 2024-03-31 15:44:46 | WARN  | main.rs:5 - This is a warning message
[my-app] 2024-03-31 15:44:46 | ERROR | main.rs:6 - This is an error message
[my-app] 2024-03-31 15:44:46 | DEBUG | main.rs:7 - This is a debug message
[my-app] 2024-03-31 15:44:46 | TRACE | main.rs:8 - This is a trace message
```

# About date and time

The logger uses a custom datetime function that doesn't depend on any external crate. This was done
to avoid adding unnecessary dependencies, but it also means that the datetime is in the UTC 
timezone.

If we want to use a different timezone, we will need to create a custom implementation or to just
use a crate like `chrono` or `time`.

That's why the logger provides a second setup function that allows us to pass a custom datetime
function. The custom function should receive no arguments and return a string with the current
datetime.

```rs
use lool::logger::{ConsoleLogger, Level};
use custom_implementation::custom_datetime_fn;

fn main() {
    ConsoleLogger::custom_setup(Level::Trace, "my-app", custom_datetime_fn).unwrap();
}
```

The library also provides a convenient function in case we just don't want to display the datetime:

```rs
use lool::logger::{ConsoleLogger, Level, datetime::noop_datetime};

fn main() {
    ConsoleLogger::custom_setup(Level::Trace, "my-app", noop_datetime).unwrap();
}
```