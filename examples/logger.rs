use lool::logger::{debug, error, info, trace, warn, ConsoleLogger, Level};

fn main() {
    ConsoleLogger::default_setup(Level::Trace, "test").unwrap();
    info!("log line");
    warn!("log line");
    error!("log line");
    debug!("log line");
    trace!("log line");
}
