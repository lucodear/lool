pub mod datetime;

use {
    eyre::{eyre, Result},
    log::{Level, Metadata, Record},
};

const RESET: &str = "\x1b[0m";

struct StyledRecord {
    time: String,
    level: String,
    message: String,
    file: String,
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

        Self {
            level: format!("{}{:<5}{}", ansi_style_level, record.level(), RESET),
            message: format!("{}", record.args()),
            file: format!(
                "{}{}{}",
                file_ansi_color,
                record.file().unwrap_or("unknown"),
                RESET
            ),
            line: format!("{}{}{}", line_ansi_color, record.line().unwrap_or(0), RESET),
            time,
        }
    }
}

pub struct ConsoleLogger<'a> {
    context: &'a str,
    time_fn: fn() -> String,
}

impl<'a> ConsoleLogger<'a> {
    pub fn default_setup(max_level: Level, context: &'static str) -> Result<()> {
        let logger = Box::new(ConsoleLogger {
            context,
            time_fn: datetime::utc_current_time,
        });
        log::set_logger(Box::leak(logger) as &'static ConsoleLogger)
            .map(|()| log::set_max_level(max_level.to_level_filter()))
            .map_err(|err| eyre!("failed to set logger: {}", err))
    }

    pub fn setup(max_level: Level, context: &'static str, time_fn: fn() -> String) -> Result<()> {
        let logger = Box::new(ConsoleLogger { context, time_fn });
        log::set_logger(Box::leak(logger) as &'static ConsoleLogger)
            .map(|()| log::set_max_level(max_level.to_level_filter()))
            .map_err(|err| eyre!("failed to set logger: {}", err))
    }

    pub fn set_context(&mut self, context: &'a str) {
        self.context = context;
    }
}

/// simple implementation of a themed logger
impl<'a> log::Log for ConsoleLogger<'a> {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        let styled_record = StyledRecord::from(record, self.time_fn);

        let time = if styled_record.time.is_empty() {
            " ".to_string()
        } else {
            format!(" {} ", styled_record.time)
        };

        // print to stdout
        println!(
            "[{}]{}| {} | {}:{} - {}",
            self.context,
            time,
            styled_record.level,
            styled_record.file,
            styled_record.line,
            styled_record.message
        );
    }

    fn flush(&self) {}
}
