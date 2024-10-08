#![doc(
    html_logo_url = "https://raw.githubusercontent.com/lucodear/lool/master/.github/img/logo.svg"
)]

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "cli.tui")]
pub use cli::tui;

#[cfg(feature = "sched")]
pub mod sched;

#[cfg(feature = "logger")]
pub mod logger;

#[cfg(feature = "macros")]
pub mod macros;

#[cfg(feature = "utils")]
pub mod utils;
