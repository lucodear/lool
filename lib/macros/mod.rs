/// 🧉 » create string
/// --
///
/// create a new `String` from a string literal or a string expression -
/// this macro is equivalent to `String::from(<args>)`.
///
/// ### Example
/// ```rust
/// use lool::s;
/// let s: String = s!("hello world");
/// ```
#[macro_export]
macro_rules! s {
    ($s:expr) => {
        String::from($s)
    };
}

/// 🧉 » fail macro
/// --
///
/// this macro is equivalent to `Err(eyre!(<args>))`.
///
/// same as `bail!` but without the explicit return
///
/// ### Example
///
/// ```should_panic
/// use lool::fail;
/// use eyre::Result;
/// # use eyre::{set_hook, DefaultHandler};
/// # fn setup_eyre() {
/// #    let _ = set_hook(Box::new(DefaultHandler::default_with));
/// # }
///
/// fn main() -> Result<()> {
///     # setup_eyre();
///     fail!("permission denied")
/// }
/// ```
#[macro_export]
macro_rules! fail {
    ($msg:literal $(,)?) => {
        Err(eyre::eyre!($msg))
    };
    ($err:expr $(,)?) => {
        Err(eyre::eyre!($err))
    };
    ($fmt:expr, $($arg:tt)*) => {
        Err(eyre::eyre!($fmt, $($arg)*))
    };
}

/// 🧉 » fail macro
/// --
///
/// this macro is equivalent to `Err(eyre!(<args>))`.
///
/// same as `bail!` but without the explicit return
///
/// this is an alias for `fail!`
///
/// ### Example
///
/// ```should_panic
/// use lool::f;
/// use eyre::Result;
/// # use eyre::{set_hook, DefaultHandler};
/// # fn setup_eyre() {
/// #    let _ = set_hook(Box::new(DefaultHandler::default_with));
/// # }
///
/// fn main() -> Result<()> {
///     # setup_eyre();
///     f!("permission denied")
/// }
/// ```
#[macro_export]
macro_rules! f {
    ($($tokens:tt)*) => {
        lool::fail!($($tokens)*)
    };
}
