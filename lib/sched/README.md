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

# Additional Features

- `sched-tokio`: Enables the `tokio` runtime support.
- `sched-rule-recurrent`: Enables the "recurrent-rule" style for scheduling tasks.
- `sched-rule-cron`: Enables the "cron-like" style for scheduling tasks

> [!WARNING] Not implemented warning
> although the `sched-rule-cron` feature is available, it's not yet implemented.

## Planned Features

- `sched-rule-cron`: Enables the "cron-like" style for scheduling tasks
- `sched-rule-pysched`: Enables the "pysched-like" style for scheduling tasks

# Usage

<!-- 
TODO 
-->
