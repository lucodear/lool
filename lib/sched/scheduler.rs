#[cfg(not(any(feature = "sched.tokio", feature = "sched.threads")))]
compile_error!("at least one of 'sched.tokio' or 'sched.threads' features must be enabled");

#[cfg(feature = "sched.tokio")]
pub mod tokio;

#[cfg(feature = "sched.threads")]
pub mod threads;
