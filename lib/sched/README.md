<p align="center"><img src="../../.github/img/logo-sched.svg" width="200"></p>

<br>
<br>
<br>

<p align="center">
<b>lool » <code>sched</code></b> is a utility library that provides a way to schedule tasks in 
various ways. Supports <code>std::thread</code> and the <a href="https://tokio.rs">tokio</a> runtime
(as a feature flag).
</p>


<br>
<br>
<br>

# Installation

This library is for internal use. And as such, it's only published privately. 

```bash
cargo add lool --registry=lugit --features sched {sub-feature}
```

# Sub-Features

## Runtimes

At least one of the following features must be enabled to use this library.

- <a href="#"><img alt="has subfeatures" src="../../.github/img/icon-has-submodules.svg" height="12"></a>&nbsp;&nbsp;
  **sched.tokio**: Enables the `tokio` runtime support.

- <a href="#"><img alt="has subfeatures" src="../../.github/img/icon-has-submodules.svg" height="12"></a>&nbsp;&nbsp;
  **sched.threads**: Enables the `std::thread` runtime support.

- <a href="#"><img alt="has subfeatures" src="../../.github/img/icon-has-submodules.svg" height="12"></a>&nbsp;&nbsp;
  **sched.thread-pool**: Enables the `std::thread` runtime support with a thread
  pool&nbsp;<img alt="unimplemented" src="../../.github/img/unimplemented.svg" height="12">

## Scheduling Rules

The default way to schedule tasks is to use a `DateTime` object. However, that means that the task
will only run once. To schedule tasks to run at specific intervals, you can use the following
features:

- <a href="#"><img alt="has subfeatures" src="../../.github/img/icon-has-submodules.svg" height="12"></a>&nbsp;&nbsp;
  **sched.rule-recurrent**: Enables the "**recurrent-rule**" style for scheduling tasks.

- <a href="#"><img alt="has subfeatures" src="../../.github/img/icon-has-submodules.svg" height="12"></a>&nbsp;&nbsp;
  **sched.rule-cron**: Enables the "cron-like" style for scheduling tasks


## Planned Features

- **sched.rule-pyschedule**: Enables the [python schedule](https://pypi.org/project/schedule/)-like
  style for scheduling tasks
- **sched.thread-pool**: Enables the `std::thread` runtime support with a thread pool (like 
  [scheduled-thread-pool](https://crates.io/crates/scheduled-thread-pool) crate but using `lool`'s
  thread pool implementation)


# Usage

Check the [examples](../../examples) directory for usage examples:

- [tokio](../../examples/sched_tokio.rs): using the `tokio` runtime
- [threads](../../examples/sched.rs): using the `std::thread` runtime

# Inspiration

This library is inspired by several other libraries, including:

- [node-schedule](https://github.com/node-schedule/node-schedule?tab=readme-ov-file#recurrence-rule-scheduling): 
  where the idea of recurring rules was taken from.
- [tokio-schedule](https://github.com/dedefer/tokio_schedule): tokio async scheduler
- [schedule-rs](https://github.com/mehcode/schedule-rs): simple thread-based scheduler
- [croner-rust](https://github.com/hexagon/croner-rust): a croner parser for rust
- [job_scheduler](https://github.com/lholden/job_scheduler): another thread-based job scheduler for
  rust
- [python's schedule](https://pypi.org/project/schedule/): a simple python scheduler with 
  a human-friendly API

