use {
    eyre::Result,
    lool::logger::{debug, error, info, trace, warn, ConsoleLogger, Level},
};

fn main() -> Result<()> {
    ConsoleLogger::builder()
        .with_name("test")
        .with_level(Level::Trace)
        .ignore("examples/logger.rs")
        .install()?;

    info!("log line");
    warn!("log line");
    error!("log line");
    debug!("log line");
    trace!("log line");

    Ok(())
}
