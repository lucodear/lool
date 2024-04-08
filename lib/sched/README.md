<p align="center"><img src="../../.github/img/logo-tokio-sched.svg" height="256"></p>

<br>
<br>
<br>

<p align="center">
<b>lool » <code>sched</code></b> is a utility library that provides a way to schedule tasks in various ways. Supports <code>std::thread</code> and the <a href="https://tokio.rs">tokio</a> runtime (as a feature flag).
</p>


<br>
<br>
<br>

# Installation

This library is for internal use. And as such, it's only published privately. 

```bash
cargo add lool --registry=lugit --features sched
```

# Sub-Features


### <picture><img alt="has subfeatures" src="../../.github/img/icon-has-submodules.svg" height="12"></picture>&nbsp;&nbsp;sched.tokio

Enables the `tokio` runtime support, replacing the default behaviour, which implies a `std::thread` pool to run the tasks.

### <picture><img alt="has subfeatures" src="../../.github/img/icon-has-submodules.svg" height="12"></picture>&nbsp;&nbsp;sched.rule-recurrent

Enables the "**recurrent-rule**" style for scheduling tasks.

### <picture><img alt="has subfeatures" src="../../.github/img/icon-has-submodules.svg" height="12"></picture>&nbsp;&nbsp;sched.rule-cron

> [!WARNING]
> 
> Not yet implemented

## Planned Features

- `sched.rule-cron`: Enables the "cron-like" style for scheduling tasks
- `sched.rule-pyschedule`: Enables the [python schedule](https://pypi.org/project/schedule/)-like
  style for scheduling tasks

# Usage

<!-- 
TODO 
-->


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


Otros:

- https://crates.io/crates/tokio-cron-scheduler
- https://crates.io/crates/clokwerk
- https://crates.io/crates/cron_tab
- https://crates.io/crates/multithreading
- https://crates.io/crates/multithreading
- https://crates.io/crates/threadpool
- https://crates.io/crates/blocking
- https://crates.io/crates/scheduled-thread-pool
- https://crates.io/crates/clokwerk
