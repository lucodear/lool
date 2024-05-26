pub mod datetime;
pub use log::{
    debug, error, info, set_max_level, trace, warn, Level, LevelFilter, Log, Metadata, Record,
};

use {
    eyre::{eyre, Result},
    std::path::Path,
};

const RESET: &str = "\x1b[0m";

struct StyledRecord {
    time: String,
    level: String,
    message: String,
    /// relative path if local file or <crate_name>::<..>::<mod_name> if
    /// it's a record created by a dependency
    ctx: String,
    raw_ctx: String,
    line: String,
}

impl StyledRecord {
    fn from(record: &Record, get_time: fn() -> String) -> Self {
        let ansi_style_level = match record.level() {
            Level::Error => "\x1b[31m", // red
            Level::Warn => "\x1b[33m",  // yellow
            Level::Info => "\x1b[32m",  // green
            Level::Debug => "\x1b[95m", // magenta
            Level::Trace => "\x1b[34m", // blue
        };

        let file_ansi_color = "\x1b[34m"; // blue
        let line_ansi_color = "\x1b[34;1m"; // bold blue
        let mut time = get_time();

        if !time.is_empty() {
            time = format!("\x1b[2m{}\x1b[0m", time); // dim
        }

        let file = Path::new(record.file().unwrap_or("unknown"));

        let line = match file.is_absolute() {
            true => String::from(""),
            false => format!("{}{}{}", line_ansi_color, record.line().unwrap_or(0), RESET),
        };

        let file = match file.is_absolute() {
            true => record.module_path().unwrap_or("unknown"),
            false => file.to_str().unwrap_or("unknown"),
        };

        Self {
            level: format!("{}{:<5}{}", ansi_style_level, record.level(), RESET),
            message: format!("{}", record.args()),
            ctx: format!("{}{}{}", file_ansi_color, file.replace('\\', "/"), RESET),
            raw_ctx: file.replace('\\', "/"),
            line,
            time,
        }
    }
}

/// 🧉 » simple console logger implementation
/// --
///
/// this is a simple logger implementation (mounted on top of the `log` crate) that logs to stdout
/// with ANSI colors and datetime stamps.
pub struct ConsoleLogger {
    name: String,
    time_fn: fn() -> String,
    ignored_ctx: Vec<String>,
}

impl ConsoleLogger {
    /// **🧉 » sets up the logger with the default settings**
    ///
    /// sets the logger to use the `datetime::utc_current_time` function to get the current time.
    pub fn default_setup<S: AsRef<str>>(max_level: Level, context: S) -> Result<()> {
        let logger = Box::new(ConsoleLogger {
            name: context.as_ref().to_string(),
            time_fn: datetime::utc_current_time,
            ignored_ctx: vec![],
        });
        log::set_logger(Box::leak(logger) as &'static dyn Log)
            .map(|()| log::set_max_level(max_level.to_level_filter()))
            .map_err(move |err| eyre!("failed to set logger: {}", err))
    }

    /// **🧉 » sets up the logger with the given settings**
    ///
    /// the `time_fn` parameter should be a function that returns a string representation of the
    /// current time.
    ///
    /// the default `time_fn` is `datetime::utc_current_time` and it doesn't take TZ into account.
    pub fn setup<S: AsRef<str>>(
        max_level: Level,
        context: S,
        time_fn: fn() -> String,
    ) -> Result<()> {
        let logger = Box::new(ConsoleLogger {
            name: context.as_ref().to_string(),
            time_fn,
            ignored_ctx: vec![],
        });
        log::set_logger(Box::leak(logger) as &'static dyn Log)
            .map(|()| log::set_max_level(max_level.to_level_filter()))
            .map_err(move |err| eyre!("failed to set logger: {}", err))
    }

    /// **🧉 » creates a new setup builder**
    ///
    /// this is the preferred way to set up the logger as it allows for more flexibility.
    ///
    /// See the `SetupBuilder` struct for more information.
    pub fn builder() -> SetupBuilder {
        SetupBuilder::default()
    }

    /// returns true if the context should be ignored
    fn should_ignore(&self, ctx: &String) -> bool {
        self.ignored_ctx.contains(ctx)
    }
}

impl Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        let styled_record = StyledRecord::from(record, self.time_fn);

        // ignore the record if the context is in the ignore list
        if self.should_ignore(&styled_record.raw_ctx) {
            return;
        }

        let time = if styled_record.time.is_empty() {
            " ".to_string()
        } else {
            format!(" {} ", styled_record.time)
        };

        let ctx = if self.name.is_empty() {
            "".to_string()
        } else {
            format!("{} »", self.name)
        };

        // print to stdout
        println!(
            "{}{}| {} | {}:{} - {}",
            ctx,
            time,
            styled_record.level,
            styled_record.ctx,
            styled_record.line,
            styled_record.message
        );
    }

    fn flush(&self) {}
}

/// **🧉 » setup builder**
///
/// this struct is used to set up the logger with more flexibility.
///
/// It allows for setting the logger name, the time function, the ignored contexts, and the max
/// level.
///
/// The `install` method is used to build and install the logger. Should be called at the end of the
/// builder chain.
///
/// **Example**
///
/// ```rust
/// use eyre::Result;
/// use lool::logger::{ConsoleLogger, Level, info};
///
/// fn main() -> Result<()> {
///   ConsoleLogger::builder()
///     .with_name("test")
///     .with_level(Level::Trace)
///     .ignore("crate::module::function") // ignore a specific context
///     .install()?;
///   
///   info!("log line");
/// }
/// ```
pub struct SetupBuilder {
    name: Option<String>,
    time_fn: Option<fn() -> String>,
    ignored_ctx: Option<Vec<String>>,
    max_level: Option<Level>,
}

impl Default for SetupBuilder {
    fn default() -> Self {
        Self {
            name: Some("".to_string()),
            time_fn: Some(datetime::utc_current_time),
            ignored_ctx: Some(vec![]),
            max_level: Some(Level::Info),
        }
    }
}

impl SetupBuilder {
    /// **🧉 » `with_name`**
    ///
    /// Sets the logger name
    pub fn with_name<S: AsRef<str>>(mut self, name: S) -> Self {
        self.name = Some(name.as_ref().to_string());
        self
    }

    /// **🧉 » `with_level`**
    ///
    /// Sets the max log level
    pub fn with_level(mut self, level: Level) -> Self {
        self.max_level = Some(level);
        self
    }

    /// **🧉 » `with_time_fn`**
    ///
    /// Sets the time function that will be used to get the current time
    pub fn with_time_fn(mut self, time_fn: fn() -> String) -> Self {
        self.time_fn = Some(time_fn);
        self
    }

    /// **🧉 » `ignore_all`**
    ///
    /// Sets the ignored contexts from a list of strings
    pub fn ignore_all(mut self, ignored_ctx: Vec<String>) -> Self {
        self.ignored_ctx = Some(ignored_ctx);
        self
    }

    /// **🧉 » `ignore`**
    ///
    /// Adds a context to the ignored list.
    ///
    /// Unlike `ignore_all`, this method allows for adding a single ignored context at a time.
    pub fn ignore<Str: AsRef<str>>(mut self, ctx: Str) -> Self {
        let mut ignored_ctx = self.ignored_ctx.unwrap();
        ignored_ctx.push(ctx.as_ref().to_string());
        self.ignored_ctx = Some(ignored_ctx);
        self
    }

    /// **🧉 » `install`**
    ///
    /// Builds and installs the logger.
    pub fn install(self) -> Result<()> {
        let logger = Box::new(ConsoleLogger {
            name: self.name.unwrap_or("".to_string()),
            time_fn: self.time_fn.unwrap_or(datetime::utc_current_time),
            ignored_ctx: self.ignored_ctx.unwrap_or(vec![]),
        });
        log::set_logger(Box::leak(logger) as &'static dyn Log)
            .map(|()| log::set_max_level(self.max_level.unwrap().to_level_filter()))
            .map_err(|err| eyre!("failed to set logger: {}", err))
    }
}
