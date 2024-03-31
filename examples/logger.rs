use {log::Level, lool::logger::ConsoleLogger};

fn main() {
    ConsoleLogger::default_setup(Level::Trace, "test").unwrap();
    log::info!("log line");
    log::warn!("log line");
    log::error!("log line");
    log::debug!("log line");
    log::trace!("log line");
}
